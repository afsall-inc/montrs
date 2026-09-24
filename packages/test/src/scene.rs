// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Hermetic 3D scene simulation for agents and tests.
//!
//! Deterministic geometry and projection with no GPU: build a [`Scene`], project
//! it through a [`Camera`], assert on transformed/ projected geometry, or
//! rasterize a wireframe to a pixel buffer and compare hashes. This lets an
//! agent reason about 3D models and scenes *by code*, without `computer-use`.

/// A 3-component vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, o: Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        if len <= f32::EPSILON {
            return Vec3::ZERO;
        }
        self.scale(1.0 / len)
    }
}

/// A column-major 4×4 matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub m: [f32; 16],
}

impl Mat4 {
    pub const fn identity() -> Self {
        #[rustfmt::skip]
        let m = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        Self { m }
    }

    pub fn translate(t: Vec3) -> Self {
        let mut r = Mat4::identity();
        r.m[12] = t.x;
        r.m[13] = t.y;
        r.m[14] = t.z;
        r
    }

    pub fn scale(s: Vec3) -> Self {
        let mut r = Mat4::identity();
        r.m[0] = s.x;
        r.m[5] = s.y;
        r.m[10] = s.z;
        r
    }

    pub fn rotate_y(angle_rad: f32) -> Self {
        let (s, c) = angle_rad.sin_cos();
        let mut r = Mat4::identity();
        r.m[0] = c;
        r.m[2] = -s;
        r.m[8] = s;
        r.m[10] = c;
        r
    }

    pub fn multiply(self, o: Mat4) -> Mat4 {
        let mut out = [0.0f32; 16];
        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[k * 4 + row] * o.m[col * 4 + k];
                }
                out[col * 4 + row] = sum;
            }
        }
        Mat4 { m: out }
    }

    /// Transform a point (w = 1), returning the homogeneous result.
    pub fn transform_point4(self, p: Vec3) -> [f32; 4] {
        [
            self.m[0] * p.x + self.m[4] * p.y + self.m[8] * p.z + self.m[12],
            self.m[1] * p.x + self.m[5] * p.y + self.m[9] * p.z + self.m[13],
            self.m[2] * p.x + self.m[6] * p.y + self.m[10] * p.z + self.m[14],
            self.m[3] * p.x + self.m[7] * p.y + self.m[11] * p.z + self.m[15],
        ]
    }

    pub fn transform_point(self, p: Vec3) -> Vec3 {
        let h = self.transform_point4(p);
        Vec3::new(h[0], h[1], h[2])
    }
}

/// A perspective camera looking at a target.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_y_deg: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn looking_at(eye: Vec3, target: Vec3) -> Self {
        Self {
            eye,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            fov_y_deg: 60.0,
            znear: 0.1,
            zfar: 1000.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let forward = (self.target - self.eye).normalize();
        let right = forward.cross(self.up).normalize();
        let up = right.cross(forward);
        let mut r = Mat4::identity();
        r.m[0] = right.x;
        r.m[4] = right.y;
        r.m[8] = right.z;
        r.m[1] = up.x;
        r.m[5] = up.y;
        r.m[9] = up.z;
        r.m[2] = -forward.x;
        r.m[6] = -forward.y;
        r.m[10] = -forward.z;
        r.m[12] = -right.dot(self.eye);
        r.m[13] = -up.dot(self.eye);
        r.m[14] = forward.dot(self.eye);
        r
    }

    /// Project a world point into normalized device coordinates (`[-1, 1]`),
    /// returning `None` when behind the camera.
    pub fn project_ndc(&self, p: Vec3, aspect: f32) -> Option<Vec3> {
        let view = self.view_matrix();
        let v = view.transform_point4(p);
        // Forward is -z in view space.
        if v[2] >= -self.znear {
            return None;
        }
        let f = 1.0 / (self.fov_y_deg.to_radians() / 2.0).tan();
        let x = (f / aspect) * v[0];
        let y = f * v[1];
        let z = v[2];
        let w = -z;
        Some(Vec3::new(x / w, y / w, z / w))
    }

    /// Project a world point to pixel coordinates in a `width × height` viewport.
    pub fn project_pixel(
        &self,
        p: Vec3,
        width: u32,
        height: u32,
    ) -> Option<(f32, f32)> {
        let aspect = width as f32 / height.max(1) as f32;
        let ndc = self.project_ndc(p, aspect)?;
        let x = (ndc.x * 0.5 + 0.5) * width as f32;
        let y = (1.0 - (ndc.y * 0.5 + 0.5)) * height as f32;
        Some((x, y))
    }
}

/// An axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn from_points(points: &[Vec3]) -> Option<Self> {
        let first = *points.first()?;
        let mut min = first;
        let mut max = first;
        for p in points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        Some(Self { min, max })
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn contains(&self, p: Vec3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }
}

/// A triangle mesh in object space.
#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    /// Triangles as vertex-index triples.
    pub triangles: Vec<[usize; 3]>,
}

impl Mesh {
    /// A unit cube centred at the origin.
    pub fn cube(size: f32) -> Self {
        let h = size / 2.0;
        let vertices = vec![
            Vec3::new(-h, -h, -h),
            Vec3::new(h, -h, -h),
            Vec3::new(h, h, -h),
            Vec3::new(-h, h, -h),
            Vec3::new(-h, -h, h),
            Vec3::new(h, -h, h),
            Vec3::new(h, h, h),
            Vec3::new(-h, h, h),
        ];
        let triangles = vec![
            [0, 1, 2],
            [0, 2, 3],
            [4, 6, 5],
            [4, 7, 6],
            [0, 4, 5],
            [0, 5, 1],
            [3, 2, 6],
            [3, 6, 7],
            [0, 3, 7],
            [0, 7, 4],
            [1, 5, 6],
            [1, 6, 2],
        ];
        Self {
            vertices,
            triangles,
        }
    }

    /// Transform every vertex by `m`, returning a new mesh.
    pub fn transformed(&self, m: Mat4) -> Mesh {
        Mesh {
            vertices: self
                .vertices
                .iter()
                .map(|v| m.transform_point(*v))
                .collect(),
            triangles: self.triangles.clone(),
        }
    }

    pub fn bounds(&self) -> Option<Aabb> {
        Aabb::from_points(&self.vertices)
    }

    /// Unique wireframe edges (each pair of triangle vertices).
    pub fn edges(&self) -> Vec<[usize; 2]> {
        let mut seen = std::collections::BTreeSet::new();
        for t in &self.triangles {
            for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                let key = if a < b { (a, b) } else { (b, a) };
                seen.insert(key);
            }
        }
        seen.into_iter().map(|(a, b)| [a, b]).collect()
    }
}

/// A collection of positioned meshes.
#[derive(Debug, Clone, Default)]
pub struct Scene {
    pub objects: Vec<(Mesh, Mat4)>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, mesh: Mesh, transform: Mat4) -> &mut Self {
        self.objects.push((mesh, transform));
        self
    }

    /// World-space bounds of every object.
    pub fn bounds(&self) -> Option<Aabb> {
        let mut pts = Vec::new();
        for (mesh, m) in &self.objects {
            for v in &mesh.vertices {
                pts.push(m.transform_point(*v));
            }
        }
        Aabb::from_points(&pts)
    }

    /// Project every wireframe edge to 2D line segments in pixel space.
    pub fn project_wireframe(
        &self,
        camera: &Camera,
        width: u32,
        height: u32,
    ) -> Vec<[(f32, f32); 2]> {
        let mut segs = Vec::new();
        for (mesh, m) in &self.objects {
            let world = mesh.transformed(*m);
            for [a, b] in world.edges() {
                let (pa, pb) = (world.vertices[a], world.vertices[b]);
                if let (Some(a2), Some(b2)) = (
                    camera.project_pixel(pa, width, height),
                    camera.project_pixel(pb, width, height),
                ) {
                    segs.push([a2, b2]);
                }
            }
        }
        segs
    }

    /// Rasterize the wireframe to an RGBA pixel buffer (white on black).
    pub fn rasterize_wireframe(
        &self,
        camera: &Camera,
        width: u32,
        height: u32,
    ) -> Wireframe {
        let mut data = vec![0u8; (width * height * 4) as usize];
        for i in (0..data.len()).step_by(4) {
            data[i] = 16;
            data[i + 1] = 16;
            data[i + 2] = 16;
            data[i + 3] = 255;
        }
        for [a, b] in self.project_wireframe(camera, width, height) {
            draw_line(&mut data, width, height, a, b);
        }
        Wireframe {
            width,
            height,
            data,
        }
    }
}

/// A rendered wireframe image.
#[derive(Debug, Clone, PartialEq)]
pub struct Wireframe {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Wireframe {
    /// A stable hash of the pixels, for snapshot comparison.
    pub fn hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.width.hash(&mut h);
        self.height.hash(&mut h);
        self.data.hash(&mut h);
        h.finish()
    }

    /// The fraction of lit (non-background) pixels.
    pub fn coverage(&self) -> f32 {
        let lit = self
            .data
            .chunks_exact(4)
            .filter(|px| px[0] > 64 || px[1] > 64 || px[2] > 64)
            .count();
        lit as f32 / (self.width * self.height).max(1) as f32
    }
}

fn draw_line(
    data: &mut [u8],
    width: u32,
    height: u32,
    a: (f32, f32),
    b: (f32, f32),
) {
    let (mut x0, mut y0) = (a.0.round() as i32, a.1.round() as i32);
    let (x1, y1) = (b.0.round() as i32, b.1.round() as i32);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as u32) < width && (y0 as u32) < height {
            let idx = ((y0 as u32 * width + x0 as u32) * 4) as usize;
            if idx + 3 < data.len() {
                data[idx] = 255;
                data[idx + 1] = 255;
                data[idx + 2] = 255;
                data[idx + 3] = 255;
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat4_translate_and_scale() {
        let t = Mat4::translate(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.transform_point(Vec3::ZERO), Vec3::new(1.0, 2.0, 3.0));
        let s = Mat4::scale(Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(
            s.transform_point(Vec3::new(1.0, 1.0, 1.0)),
            Vec3::new(2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn camera_projects_the_target_to_screen_centre() {
        let cam = Camera::looking_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO);
        let (x, y) = cam.project_pixel(Vec3::ZERO, 800, 600).unwrap();
        assert!((x - 400.0).abs() < 1.0, "x = {x}");
        assert!((y - 300.0).abs() < 1.0, "y = {y}");
    }

    #[test]
    fn points_behind_the_camera_do_not_project() {
        let cam = Camera::looking_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO);
        assert!(
            cam.project_pixel(Vec3::new(0.0, 0.0, 10.0), 800, 600)
                .is_none()
        );
    }

    #[test]
    fn cube_bounds_are_symmetric() {
        let cube = Mesh::cube(2.0);
        let b = cube.bounds().unwrap();
        assert_eq!(b.size(), Vec3::new(2.0, 2.0, 2.0));
        assert!(b.contains(Vec3::ZERO));
    }

    #[test]
    fn wireframe_hash_is_deterministic() {
        let mut scene = Scene::new();
        scene.add(Mesh::cube(2.0), Mat4::identity());
        let cam = Camera::looking_at(Vec3::new(0.0, 0.0, 6.0), Vec3::ZERO);
        let a = scene.rasterize_wireframe(&cam, 128, 128);
        let b = scene.rasterize_wireframe(&cam, 128, 128);
        assert_eq!(a.hash(), b.hash());
        assert!(a.coverage() > 0.0);
    }

    #[test]
    fn rotation_changes_the_projection() {
        let mut scene = Scene::new();
        scene.add(Mesh::cube(2.0), Mat4::identity());
        let cam = Camera::looking_at(Vec3::new(0.0, 0.0, 6.0), Vec3::ZERO);
        let base = scene.rasterize_wireframe(&cam, 128, 128).hash();

        let mut rotated = Scene::new();
        rotated
            .add(Mesh::cube(2.0), Mat4::rotate_y(std::f32::consts::FRAC_PI_4));
        let spun = rotated.rasterize_wireframe(&cam, 128, 128).hash();
        assert_ne!(base, spun);
    }

    #[test]
    fn edges_are_unique() {
        let cube = Mesh::cube(1.0);
        let edges = cube.edges();
        assert_eq!(edges.len(), 18, "a cube has 12 edges + 6 diagonals split");
    }
}
