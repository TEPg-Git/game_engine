// ============================================================
// TEXTURE SYSTEM
// ============================================================

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    // ========================================================
    // LOAD IMAGE FROM FILE
    // ========================================================

    pub fn from_file(device: &wgpu::Device, queue: &wgpu::Queue, path: &str) -> Self {
        // ----------------------------------------------------
        // LOAD IMAGE
        // ----------------------------------------------------

        let img = image::open(path).unwrap_or_else(|_| panic!("Failed to load image: {}", path));

        // ----------------------------------------------------
        // CONVERT TO RGBA
        // ----------------------------------------------------

        let rgba = img.to_rgba8();

        let dimensions = rgba.dimensions();

        // ----------------------------------------------------
        // TEXTURE SIZE
        // ----------------------------------------------------

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        // ----------------------------------------------------
        // CREATE GPU TEXTURE
        // ----------------------------------------------------

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Image Texture"),

            size,

            mip_level_count: 1,

            sample_count: 1,

            dimension: wgpu::TextureDimension::D2,

            format: wgpu::TextureFormat::Rgba8UnormSrgb,

            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,

            view_formats: &[],
        });

        // ----------------------------------------------------
        // COPY IMAGE INTO GPU TEXTURE
        // ----------------------------------------------------

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,

                mip_level: 0,

                origin: wgpu::Origin3d::ZERO,

                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,

                bytes_per_row: Some(4 * dimensions.0),

                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // ----------------------------------------------------
        // TEXTURE VIEW
        // ----------------------------------------------------

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ----------------------------------------------------
        // SAMPLER
        // ----------------------------------------------------

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Image Sampler"),

            address_mode_u: wgpu::AddressMode::ClampToEdge,

            address_mode_v: wgpu::AddressMode::ClampToEdge,

            address_mode_w: wgpu::AddressMode::ClampToEdge,

            mag_filter: wgpu::FilterMode::Linear,

            min_filter: wgpu::FilterMode::Linear,

            mipmap_filter: wgpu::MipmapFilterMode::Linear,

            ..Default::default()
        });

        // ----------------------------------------------------
        // RETURN TEXTURE
        // ----------------------------------------------------

        Self {
            texture,

            view,

            sampler,
        }
    }
}
