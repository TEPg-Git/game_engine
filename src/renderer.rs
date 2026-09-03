use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::graphics::{Uniforms, Vertex};
use crate::sprite::Sprite;
use crate::text::{create_text_bitmap, load_font};

// ============================================================
// RENDERER
// ============================================================

pub struct Renderer {
    // ========================================================
    // WGPU
    // ========================================================
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    pub texture_bind_group_layout: wgpu::BindGroupLayout,

    // ========================================================
    // UNIFORM
    // ========================================================
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,

    // ========================================================
    // TEXT
    // ========================================================
    pub text_texture: wgpu::Texture,
    pub text_bind_group: wgpu::BindGroup,
    pub text_vertex_buffer: wgpu::Buffer,
    pub text_vertex_count: u32,

    // ========================================================
    // PIPELINE
    // ========================================================
    pub render_pipeline: wgpu::RenderPipeline,
}

// ============================================================
// IMPLEMENTATION
// ============================================================

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        // ====================================================
        // WGPU INSTANCE
        // ====================================================

        let instance = wgpu::Instance::default();

        // ====================================================
        // SURFACE
        // ====================================================

        let surface = instance.create_surface(window.clone()).unwrap();

        // ====================================================
        // ADAPTER
        // ====================================================

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),

            force_fallback_adapter: false,

            compatible_surface: Some(&surface),

            apply_limit_buckets: false,
        }))
        .expect("Failed to find suitable GPU adapter");

        println!("GPU: {:?}", adapter.get_info());

        // ====================================================
        // DEVICE + QUEUE
        // ====================================================

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("East Engine Device"),

            required_features: wgpu::Features::empty(),

            required_limits: wgpu::Limits::default(),

            experimental_features: wgpu::ExperimentalFeatures::disabled(),

            memory_hints: wgpu::MemoryHints::default(),

            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create GPU device");

        println!("Device created!");

        // ====================================================
        // SURFACE CONFIG
        // ====================================================

        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface is not supported");

        surface.configure(&device, &config);

        println!("Surface configured!");

        // ====================================================
        // UNIFORM BUFFER
        // ====================================================

        let uniforms = Uniforms {
            position_rotation: [0.0, 0.0, 0.0, 0.0],

            scale: [1.0, 1.0, 0.0, 0.0],

            color: [1.0, 0.0, 0.0, 1.0],

            camera_position: [0.0, 0.0],

            camera_zoom: [1.0, 0.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),

            contents: bytemuck::bytes_of(&uniforms),

            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        println!("Uniform buffer created!");

        // ====================================================
        // UNIFORM BIND GROUP LAYOUT
        // ====================================================

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),

                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,

                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,

                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,

                        has_dynamic_offset: false,

                        min_binding_size: None,
                    },

                    count: None,
                }],
            });

        // ====================================================
        // UNIFORM BIND GROUP
        // ====================================================

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),

            layout: &uniform_bind_group_layout,

            entries: &[wgpu::BindGroupEntry {
                binding: 0,

                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        println!("Uniform bind group created!");

        // ====================================================
        // LOAD FONT
        // ====================================================

        let font = load_font();

        println!("Font loaded successfully!");

        // ====================================================
        // TEXT
        // ====================================================

        let text = "Hello East Engine";

        let font_size = 24.0;

        println!("Rendering text: {}", text);

        // ====================================================
        // CREATE TEXT BITMAP
        // ====================================================

        let (rgba_data, text_width, text_height) = create_text_bitmap(&font, text, font_size);

        println!("Text size: {}x{}", text_width, text_height);

        // ====================================================
        // WGPU ROW PADDING
        // ====================================================

        let unpadded_bytes_per_row = text_width * 4;

        let padded_bytes_per_row = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
            * ((unpadded_bytes_per_row + wgpu::COPY_BYTES_PER_ROW_ALIGNMENT - 1)
                / wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);

        // ====================================================
        // PAD RGBA DATA
        // ====================================================

        let mut padded_data = vec![0u8; (padded_bytes_per_row * text_height) as usize];

        for y in 0..text_height {
            let source_start = (y * unpadded_bytes_per_row) as usize;

            let source_end = source_start + unpadded_bytes_per_row as usize;

            let destination_start = (y * padded_bytes_per_row) as usize;

            let destination_end = destination_start + unpadded_bytes_per_row as usize;

            padded_data[destination_start..destination_end]
                .copy_from_slice(&rgba_data[source_start..source_end]);
        }

        // ====================================================
        // CREATE TEXTURE
        // ====================================================

        let text_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Text Texture"),

            size: wgpu::Extent3d {
                width: text_width,

                height: text_height,

                depth_or_array_layers: 1,
            },

            mip_level_count: 1,

            sample_count: 1,

            dimension: wgpu::TextureDimension::D2,

            format: wgpu::TextureFormat::Rgba8UnormSrgb,

            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,

            view_formats: &[],
        });

        // ====================================================
        // UPLOAD TEXTURE
        // ====================================================

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &text_texture,

                mip_level: 0,

                origin: wgpu::Origin3d::ZERO,

                aspect: wgpu::TextureAspect::All,
            },
            &padded_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,

                bytes_per_row: Some(padded_bytes_per_row),

                rows_per_image: Some(text_height),
            },
            wgpu::Extent3d {
                width: text_width,

                height: text_height,

                depth_or_array_layers: 1,
            },
        );

        println!("Text uploaded to GPU!");

        // ====================================================
        // TEXTURE VIEW
        // ====================================================

        let text_view = text_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ====================================================
        // SAMPLER
        // ====================================================

        let text_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Text Sampler"),

            mag_filter: wgpu::FilterMode::Linear,

            min_filter: wgpu::FilterMode::Linear,

            mipmap_filter: wgpu::MipmapFilterMode::Nearest,

            ..Default::default()
        });

        // ====================================================
        // TEXTURE BIND GROUP LAYOUT
        // ====================================================

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),

                entries: &[
                    // ========================================
                    // TEXTURE
                    // ========================================
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,

                        visibility: wgpu::ShaderStages::FRAGMENT,

                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },

                            view_dimension: wgpu::TextureViewDimension::D2,

                            multisampled: false,
                        },

                        count: None,
                    },
                    // ========================================
                    // SAMPLER
                    // ========================================
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,

                        visibility: wgpu::ShaderStages::FRAGMENT,

                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),

                        count: None,
                    },
                ],
            });

        // ====================================================
        // TEXT BIND GROUP
        // ====================================================

        let text_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Bind Group"),

            layout: &texture_bind_group_layout,

            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,

                    resource: wgpu::BindingResource::TextureView(&text_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,

                    resource: wgpu::BindingResource::Sampler(&text_sampler),
                },
            ],
        });

        println!("Text bind group created!");

        // ====================================================
        // TEXT SIZE IN SCREEN SPACE
        // ====================================================

        let screen_width = size.width as f32;

        let screen_height = size.height as f32;

        let text_width_ndc = (text_width as f32 / screen_width) * 2.0;

        let text_height_ndc = (text_height as f32 / screen_height) * 2.0;

        let half_width = text_width_ndc / 2.0;

        let half_height = text_height_ndc / 2.0;

        // ====================================================
        // TEXT QUAD
        // ====================================================

        let text_vertices = [
            Vertex {
                position: [-half_width, half_height],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [half_width, half_height],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-half_width, -half_height],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [half_width, half_height],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [half_width, -half_height],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-half_width, -half_height],
                tex_coords: [0.0, 1.0],
            },
        ];

        // ====================================================
        // TEXT VERTEX BUFFER
        // ====================================================

        let text_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Text Vertex Buffer"),

            contents: bytemuck::cast_slice(&text_vertices),

            usage: wgpu::BufferUsages::VERTEX,
        });

        let text_vertex_count = text_vertices.len() as u32;

        println!("Text vertex buffer created!");

        // ====================================================
        // SHADER
        // ====================================================

        let shader_source = include_str!("shader.wgsl");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("East Engine Shader"),

            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        println!("Shader created!");

        // ====================================================
        // PIPELINE LAYOUT
        // ====================================================

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("East Engine Pipeline Layout"),

            bind_group_layouts: &[
                Some(&uniform_bind_group_layout),
                Some(&texture_bind_group_layout),
            ],

            immediate_size: 0,
        });

        println!("Pipeline layout created!");

        // ====================================================
        // RENDER PIPELINE
        // ====================================================

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("East Engine Render Pipeline"),

            layout: Some(&pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader,

                entry_point: Some("vs_main"),

                compilation_options: wgpu::PipelineCompilationOptions::default(),

                buffers: &[Some(Vertex::layout())],
            },

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,

                strip_index_format: None,

                front_face: wgpu::FrontFace::Ccw,

                cull_mode: None,

                unclipped_depth: false,

                polygon_mode: wgpu::PolygonMode::Fill,

                conservative: false,
            },

            depth_stencil: None,

            multisample: wgpu::MultisampleState {
                count: 1,

                mask: !0,

                alpha_to_coverage_enabled: false,
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,

                entry_point: Some("fs_main"),

                compilation_options: wgpu::PipelineCompilationOptions::default(),

                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,

                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),

                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            multiview_mask: None,

            cache: None,
        });

        println!("Render pipeline created!");

        // ====================================================
        // RETURN RENDERER
        // ====================================================

        Self {
            surface,

            device,

            queue,

            config,

            texture_bind_group_layout,

            uniform_buffer,

            uniform_bind_group,

            text_texture,

            text_bind_group,

            text_vertex_buffer,

            text_vertex_count,

            render_pipeline,
        }
    }

    pub fn create_sprite_bind_group(&self, sprite: &Sprite) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sprite Bind Group"),

            layout: &self.texture_bind_group_layout,

            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,

                    resource: wgpu::BindingResource::TextureView(&sprite.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,

                    resource: wgpu::BindingResource::Sampler(&sprite.texture.sampler),
                },
            ],
        })
    }

    pub fn create_sprite_vertex_buffer(&self, sprite: &Sprite) -> (wgpu::Buffer, u32) {
        let half_width = sprite.size[0] / 2.0;
        let half_height = sprite.size[1] / 2.0;

        let vertices = [
            Vertex {
                position: [-half_width, half_height],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [half_width, half_height],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-half_width, -half_height],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [half_width, half_height],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [half_width, -half_height],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-half_width, -half_height],
                tex_coords: [0.0, 1.0],
            },
        ];

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Sprite Vertex Buffer"),

                contents: bytemuck::cast_slice(&vertices),

                usage: wgpu::BufferUsages::VERTEX,
            });

        let vertex_count = vertices.len() as u32;

        (vertex_buffer, vertex_count)
    }
}
