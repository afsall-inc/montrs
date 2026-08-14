use crate::{
    Frame, Paint, Path, Point, Quad, Rect, Renderer, Shape, Stroke, Viewport,
    pipeline::{
        image::ImagePipeline, quad::QuadPipeline, svg::SvgPipeline,
        text::TextPipeline,
    },
};
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[allow(dead_code)]
struct LayerState {
    alpha: f32,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
    quad_pipeline: QuadPipeline,
}

pub struct WgpuRenderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    viewport: Viewport,
    quad_pipeline: QuadPipeline,
    text_pipeline: TextPipeline,
    image_pipeline: ImagePipeline,
    svg_pipeline: SvgPipeline,
    current_frame: Option<wgpu::SurfaceTexture>,
    current_encoder: Option<wgpu::CommandEncoder>,
    current_view: Option<wgpu::TextureView>,
    layer_stack: Vec<LayerState>,
    clip_rect: Option<Rect>,
    quad_batch: Vec<(Quad, Paint)>,
}

impl<'a> WgpuRenderer<'a> {
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        let formats = surface.get_capabilities(&adapter).formats;
        let format = formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 800,
            height: 600,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let viewport = Viewport::new(800.0, 600.0, 1.0);
        let quad_pipeline = QuadPipeline::new(&device, &config);
        let text_pipeline = TextPipeline::new(&device, &queue, config.format);
        let image_pipeline = ImagePipeline::new(&device);
        let svg_pipeline = SvgPipeline::new();

        Self {
            surface,
            device,
            queue,
            config,
            viewport,
            quad_pipeline,
            text_pipeline,
            image_pipeline,
            svg_pipeline,
            current_frame: None,
            current_encoder: None,
            current_view: None,
            layer_stack: Vec::new(),
            clip_rect: None,
            quad_batch: Vec::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.viewport = Viewport::new(width as f32, height as f32, 1.0);
        self.text_pipeline
            .resize(&self.device, &self.queue, width, height);
    }
}

impl Renderer for WgpuRenderer<'_> {
    fn begin(&mut self, _viewport: &Viewport) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Frame Encoder"),
            },
        );
        self.current_frame = Some(frame);
        self.current_view = Some(view);
        self.current_encoder = Some(encoder);
        self.quad_batch.clear();
    }

    fn fill_quad(&mut self, quad: &Quad, paint: &Paint) {
        if self.current_view.is_none() {
            return;
        }
        self.quad_batch.push((quad.clone(), paint.clone()));
    }

    fn fill_path(&mut self, path: &Path, _paint: &Paint) {
        let Some(ref mut encoder) = self.current_encoder else {
            return;
        };
        let Some(ref view) = self.current_view else {
            return;
        };

        use lyon::{
            math::Point as LPoint,
            tessellation::{
                FillOptions, FillTessellator, VertexBuffers,
                geometry_builder::simple_builder,
            },
        };

        let mut lyon_path = lyon::path::Path::builder();
        for segment in &path.segments {
            match segment {
                crate::PathSegment::MoveTo(p) => {
                    lyon_path.begin(LPoint::new(p.x, p.y));
                }
                crate::PathSegment::LineTo(p) => {
                    lyon_path.line_to(LPoint::new(p.x, p.y));
                }
                crate::PathSegment::QuadTo(c, p) => {
                    lyon_path.quadratic_bezier_to(
                        LPoint::new(c.x, c.y),
                        LPoint::new(p.x, p.y),
                    );
                }
                crate::PathSegment::CubicTo(c1, c2, p) => {
                    lyon_path.cubic_bezier_to(
                        LPoint::new(c1.x, c1.y),
                        LPoint::new(c2.x, c2.y),
                        LPoint::new(p.x, p.y),
                    );
                }
                crate::PathSegment::Close => {
                    lyon_path.close();
                }
            }
        }
        let lyon_path = lyon_path.build();

        let mut geometry: VertexBuffers<LPoint, u16> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();
        let _ = tessellator.tessellate_path(
            &lyon_path,
            &FillOptions::default(),
            &mut simple_builder(&mut geometry),
        );

        if geometry.vertices.is_empty() {
            return;
        }

        let verts: Vec<[f32; 2]> =
            geometry.vertices.iter().map(|p| [p.x, p.y]).collect();
        let vertex_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Path Fill Vertex Buffer"),
                    contents: bytemuck::cast_slice(&verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Path Fill Index Buffer"),
                    contents: bytemuck::cast_slice(&geometry.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Path Fill Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        rp.set_pipeline(self.quad_pipeline.get_pipeline());
        rp.set_vertex_buffer(0, vertex_buffer.slice(..));
        rp.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rp.draw_indexed(0..geometry.indices.len() as u32, 0, 0..1);
    }

    fn stroke_path(&mut self, path: &Path, stroke: &Stroke, _paint: &Paint) {
        let Some(ref mut encoder) = self.current_encoder else {
            return;
        };
        let Some(ref view) = self.current_view else {
            return;
        };

        use lyon::{
            math::Point as LPoint,
            tessellation::{
                StrokeOptions, StrokeTessellator, VertexBuffers,
                geometry_builder::simple_builder,
            },
        };

        let mut lyon_path = lyon::path::Path::builder();
        for segment in &path.segments {
            match segment {
                crate::PathSegment::MoveTo(p) => {
                    lyon_path.begin(LPoint::new(p.x, p.y));
                }
                crate::PathSegment::LineTo(p) => {
                    lyon_path.line_to(LPoint::new(p.x, p.y));
                }
                crate::PathSegment::QuadTo(c, p) => {
                    lyon_path.quadratic_bezier_to(
                        LPoint::new(c.x, c.y),
                        LPoint::new(p.x, p.y),
                    );
                }
                crate::PathSegment::CubicTo(c1, c2, p) => {
                    lyon_path.cubic_bezier_to(
                        LPoint::new(c1.x, c1.y),
                        LPoint::new(c2.x, c2.y),
                        LPoint::new(p.x, p.y),
                    );
                }
                crate::PathSegment::Close => {
                    lyon_path.close();
                }
            }
        }
        let lyon_path = lyon_path.build();

        let line_cap = match stroke.line_cap {
            crate::LineCap::Butt => lyon::tessellation::LineCap::Butt,
            crate::LineCap::Round => lyon::tessellation::LineCap::Round,
            crate::LineCap::Square => lyon::tessellation::LineCap::Square,
        };
        let line_join = match stroke.line_join {
            crate::LineJoin::Miter => lyon::tessellation::LineJoin::Miter,
            crate::LineJoin::Round => lyon::tessellation::LineJoin::Round,
            crate::LineJoin::Bevel => lyon::tessellation::LineJoin::Bevel,
        };

        let mut geometry: VertexBuffers<LPoint, u16> = VertexBuffers::new();
        let mut tessellator = StrokeTessellator::new();
        let _ = tessellator.tessellate_path(
            &lyon_path,
            &StrokeOptions::default()
                .with_line_width(stroke.width)
                .with_line_cap(line_cap)
                .with_line_join(line_join),
            &mut simple_builder(&mut geometry),
        );

        if geometry.vertices.is_empty() {
            return;
        }

        let verts: Vec<[f32; 2]> =
            geometry.vertices.iter().map(|p| [p.x, p.y]).collect();
        let vertex_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Stroke Vertex Buffer"),
                    contents: bytemuck::cast_slice(&verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Stroke Index Buffer"),
                    contents: bytemuck::cast_slice(&geometry.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Stroke Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        rp.set_pipeline(self.quad_pipeline.get_pipeline());
        rp.set_vertex_buffer(0, vertex_buffer.slice(..));
        rp.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rp.draw_indexed(0..geometry.indices.len() as u32, 0, 0..1);
    }

    fn draw_glyphs(
        &mut self,
        pos: Point,
        glyphs: &[crate::GlyphRun],
        paint: &Paint,
    ) {
        if glyphs.is_empty() {
            return;
        }
        for g in glyphs {
            let first_pos = g
                .positions
                .first()
                .copied()
                .unwrap_or(Point { x: 0.0, y: 0.0 });
            self.text_pipeline.push(
                pos.x + first_pos.x,
                pos.y + first_pos.y,
                &format!("[glyph run: {} glyphs]", g.glyph_ids.len()),
                g.font_size,
                [paint.color.r, paint.color.g, paint.color.b, paint.color.a],
            );
        }
    }

    fn draw_image(&mut self, image: &crate::Image, rect: Rect) {
        let id = self.image_pipeline.upload(
            &self.device,
            &self.queue,
            &image.data,
            image.width,
            image.height,
        );
        let Some(ref mut encoder) = self.current_encoder else {
            return;
        };
        let Some(ref view) = self.current_view else {
            return;
        };
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Image Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        self.image_pipeline.draw(
            &mut rp,
            id,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            1.0,
        );
    }

    fn draw_svg(&mut self, svg: &crate::Svg, rect: Rect) {
        let id = self.svg_pipeline.render(
            &svg.data,
            rect.width as u32,
            rect.height as u32,
        );
        if id == 0 {
            return;
        }
        let Some(ref mut encoder) = self.current_encoder else {
            return;
        };
        let Some(ref view) = self.current_view else {
            return;
        };
        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("SVG Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
    }

    fn clip(&mut self, shape: &Shape) {
        let bounds = shape.path.segments.iter().fold(
            (f32::MAX, f32::MAX, f32::MIN, f32::MIN),
            |(min_x, min_y, max_x, max_y), seg| match seg {
                crate::PathSegment::MoveTo(p)
                | crate::PathSegment::LineTo(p) => (
                    min_x.min(p.x),
                    min_y.min(p.y),
                    max_x.max(p.x),
                    max_y.max(p.y),
                ),
                crate::PathSegment::QuadTo(c, p) => (
                    min_x.min(c.x).min(p.x),
                    min_y.min(c.y).min(p.y),
                    max_x.max(c.x).max(p.x),
                    max_y.max(c.y).max(p.y),
                ),
                crate::PathSegment::CubicTo(c1, c2, p) => (
                    min_x.min(c1.x).min(c2.x).min(p.x),
                    min_y.min(c1.y).min(c2.y).min(p.y),
                    max_x.max(c1.x).max(c2.x).max(p.x),
                    max_y.max(c1.y).max(c2.y).max(p.y),
                ),
                crate::PathSegment::Close => (min_x, min_y, max_x, max_y),
            },
        );
        self.clip_rect = Some(Rect {
            x: bounds.0,
            y: bounds.1,
            width: bounds.2 - bounds.0,
            height: bounds.3 - bounds.1,
        });
    }

    fn clear_clip(&mut self) {
        self.clip_rect = None;
    }

    fn push_layer(&mut self, alpha: f32, _transform: &[f32; 6]) {
        let size = self.viewport.physical_size();
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Layer Texture"),
            size: wgpu::Extent3d {
                width: size.0.max(1),
                height: size.1.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Layer Encoder"),
            },
        );
        let quad_pipeline = QuadPipeline::new(&self.device, &self.config);
        self.layer_stack.push(LayerState {
            alpha,
            texture,
            view,
            encoder,
            quad_pipeline,
        });
    }

    fn pop_layer(&mut self) {
        if let Some(_layer) = self.layer_stack.pop() {}
    }

    fn finish(&mut self) -> Frame {
        if let Some(ref mut encoder) = self.current_encoder {
            let view = self.current_view.as_ref().unwrap();
            let mut rp =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.1,
                                    b: 0.1,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

            let batch = std::mem::take(&mut self.quad_batch);
            for (quad, paint) in &batch {
                self.quad_pipeline.push(quad, paint);
            }
            self.quad_pipeline.flush(&self.device, &self.queue, &mut rp);
        }

        if let Some(encoder) = self.current_encoder.take() {
            self.queue.submit(std::iter::once(encoder.finish()));
        }

        if let Some(frame) = self.current_frame.take() {
            frame.present();
        }

        let size = self.viewport.physical_size();
        Frame {
            data: Vec::new(),
            width: size.0,
            height: size.1,
        }
    }
}
