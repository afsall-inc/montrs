//! wgpu GPU backend for the MontRS renderer.
//!
//! Implements the `Renderer` trait using wgpu (Vulkan/Metal/DX12/WebGPU).
//! Inspired by Floem's vello backend and iced's wgpu compositor.

use crate::{Frame, Paint, Path, Point, Quad, Rect, Renderer, Shape, Stroke, Viewport};
use std::sync::Arc;

/// GPU-accelerated renderer using wgpu.
pub struct WgpuRenderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    viewport: Viewport,
}

impl<'a> WgpuRenderer<'a> {
    /// Create a new wgpu renderer from a winit window.
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
        let size = surface.get_capabilities(&adapter).formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: size,
            width: 800,
            height: 600,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let viewport = Viewport::new(800.0, 600.0, 1.0);

        Self {
            surface,
            device,
            queue,
            config,
            viewport,
        }
    }

    /// Resize the surface when the window changes.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.viewport = Viewport::new(width as f32, height as f32, 1.0);
    }
}

impl Renderer for WgpuRenderer<'_> {
    fn begin(&mut self, _viewport: &Viewport) {
        // Frame is started in finish() when we get the surface texture
    }

    fn fill_quad(&mut self, _quad: &Quad, _paint: &Paint) {
        // TODO: Implement quad rendering pipeline with wgpu
    }

    fn fill_path(&mut self, _path: &Path, _paint: &Paint) {
        // TODO: Implement path tessellation and rendering
    }

    fn stroke_path(&mut self, _path: &Path, _stroke: &Stroke, _paint: &Paint) {
        // TODO: Implement path stroking
    }

    fn draw_glyphs(&mut self, _pos: Point, _glyphs: &[crate::GlyphRun], _paint: &Paint) {
        // TODO: Implement text rendering via cosmic-text + glyphon
    }

    fn draw_image(&mut self, _image: &crate::Image, _rect: Rect) {
        // TODO: Implement image rendering
    }

    fn draw_svg(&mut self, _svg: &crate::Svg, _rect: Rect) {
        // TODO: Implement SVG rendering via resvg
    }

    fn clip(&mut self, _shape: &Shape) {
        // TODO: Implement clipping
    }

    fn clear_clip(&mut self) {
        // TODO: Implement clip clearing
    }

    fn push_layer(&mut self, _alpha: f32, _transform: &[f32; 6]) {
        // TODO: Implement layer compositing
    }

    fn pop_layer(&mut self) {
        // TODO: Implement layer pop
    }

    fn finish(&mut self) -> Frame {
        let frame = self.surface.get_current_texture().expect("Failed to get surface texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Frame {
            data: Vec::new(),
            width: self.viewport.physical_size().0,
            height: self.viewport.physical_size().1,
        }
    }
}