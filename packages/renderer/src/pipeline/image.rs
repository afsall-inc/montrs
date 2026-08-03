use std::collections::HashMap;

pub struct TextureEntry {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

pub struct ImagePipeline {
    textures: HashMap<u64, TextureEntry>,
    sampler: wgpu::Sampler,
    next_id: u64,
}

impl ImagePipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Image Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            textures: HashMap::new(),
            sampler,
            next_id: 1,
        }
    }

    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> u64 {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let id = self.next_id;
        self.next_id += 1;
        self.textures.insert(
            id,
            TextureEntry {
                texture,
                view,
                width,
                height,
            },
        );
        id
    }

    #[cfg(feature = "images")]
    pub fn upload_from_image(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
    ) -> u64 {
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        self.upload(device, queue, &rgba, w, h)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        _render_pass: &mut wgpu::RenderPass,
        _id: u64,
        _dest_x: f32,
        _dest_y: f32,
        _dest_w: f32,
        _dest_h: f32,
        _opacity: f32,
    ) {
    }

    pub fn get_view(&self, id: u64) -> Option<&wgpu::TextureView> {
        self.textures.get(&id).map(|e| &e.view)
    }

    pub fn get_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
