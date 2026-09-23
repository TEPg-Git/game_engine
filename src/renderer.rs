use std::collections::HashMap;
use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::entity::Entity;
use crate::graphics::{Uniforms, Vertex};
use crate::sprite::Sprite;
use crate::state::GameState;
use crate::text::{Text, create_text_bitmap_with_options, load_font};

// ============================================================
// RENDER OBJECT
// ============================================================

pub struct RenderObject {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub sprite_bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
}

// ============================================================
// TEXT OBJECT
// ============================================================

pub struct TextObject {
    pub text: Text,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub text_texture: wgpu::Texture,
    pub text_bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub revision: u64,
}

// ============================================================
// RENDERER
// ============================================================

pub struct Renderer {
    // CORE GPU
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    // SHARED GPU LAYOUTS
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,

    // PIPELINE
    pub render_pipeline: wgpu::RenderPipeline,

    // PENDING SURFACE SIZE
    pending_width: u32,
    pending_height: u32,
    resize_pending: bool,

    // RENDER OBJECTS
    pub render_objects: HashMap<u32, RenderObject>,
    pub text_objects: HashMap<u32, TextObject>,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            apply_limit_buckets: false,
        }))
        .expect("Failed to find suitable GPU adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("East Engine Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create GPU device");

        let size = window.inner_size();

        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("Surface is not supported");

        // Do not let the render loop block on the monitor's vblank queue.
        // Windows enters a nested modal loop during live resize/fullscreen
        // transitions; FIFO presentation can stall get_current_texture()
        // while that loop is active.
        config.present_mode = wgpu::PresentMode::AutoNoVsync;
        config.desired_maximum_frame_latency = 1;

        surface.configure(&device, &config);

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

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
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
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let shader_source = include_str!("shader.wgsl");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("East Engine Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("East Engine Pipeline Layout"),
            bind_group_layouts: &[
                Some(&uniform_bind_group_layout),
                Some(&texture_bind_group_layout),
            ],
            immediate_size: 0,
        });

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

        Self {
            surface,
            device,
            queue,
            pending_width: config.width,
            pending_height: config.height,
            resize_pending: false,
            config,
            texture_bind_group_layout,
            uniform_bind_group_layout,
            render_pipeline,
            render_objects: HashMap::new(),
            text_objects: HashMap::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        // Configure immediately so the next redraw uses the exact surface size.
        self.surface.configure(&self.device, &self.config);
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.pending_width = width;
        self.pending_height = height;
        self.resize_pending = true;
    }

    pub fn apply_pending_resize(&mut self) {
        if !self.resize_pending {
            return;
        }

        self.config.width = self.pending_width.max(1);
        self.config.height = self.pending_height.max(1);
        self.surface.configure(&self.device, &self.config);
        self.resize_pending = false;
    }

    pub fn create_render_object(&mut self, entity: &Entity) {
        if let Some(sprite) = &entity.sprite {
            let uniforms = Uniforms {
                position_rotation: [0.0, 0.0, 0.0, 0.0],
                scale: [1.0, 1.0, 0.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                camera_position: [0.0, 0.0],
                camera_zoom: [1.0, 0.0],
            };

            let uniform_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Entity Uniform Buffer"),
                        contents: bytemuck::bytes_of(&uniforms),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    });

            let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Entity Uniform Bind Group"),
                layout: &self.uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

            let sprite_bind_group = self.create_sprite_bind_group(sprite);
            let (vertex_buffer, vertex_count) = self.create_sprite_vertex_buffer(sprite);

            self.render_objects.insert(
                entity.id,
                RenderObject {
                    uniform_buffer,
                    uniform_bind_group,
                    sprite_bind_group,
                    vertex_buffer,
                    vertex_count,
                },
            );
        }
    }

    fn create_sprite_bind_group(&self, sprite: &Sprite) -> wgpu::BindGroup {
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

    fn create_sprite_vertex_buffer(&self, sprite: &Sprite) -> (wgpu::Buffer, u32) {
        let half_width = sprite.size[0] / 2.0;
        let half_height = sprite.size[1] / 2.0;

        let vertices = Self::quad_vertices(half_width, half_height);

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Sprite Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        (vertex_buffer, vertices.len() as u32)
    }

    fn quad_vertices(half_width: f32, half_height: f32) -> [Vertex; 6] {
        [
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
        ]
    }

    pub fn render(&mut self, state: &GameState) {
        if self.config.width == 0 || self.config.height == 0 {
            return;
        }

        self.update_text_object(0, &state.text);
        self.update_text_object(1, &state.score_text);

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,
            wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return,
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for entity in &state.entities {
                let Some(render_object) = self.render_objects.get(&entity.id) else {
                    continue;
                };

                let uniforms = Uniforms {
                    position_rotation: [
                        entity.transform.position[0],
                        entity.transform.position[1],
                        entity.transform.rotation,
                        0.0,
                    ],
                    scale: [
                        entity.transform.scale[0],
                        entity.transform.scale[1],
                        0.0,
                        0.0,
                    ],
                    color: [1.0, 1.0, 1.0, 1.0],
                    camera_position: [state.camera.position[0], state.camera.position[1]],
                    camera_zoom: [state.camera.zoom, 0.0],
                };

                self.queue.write_buffer(
                    &render_object.uniform_buffer,
                    0,
                    bytemuck::bytes_of(&uniforms),
                );

                render_pass.set_bind_group(0, &render_object.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &render_object.sprite_bind_group, &[]);
                render_pass.set_vertex_buffer(0, render_object.vertex_buffer.slice(..));
                render_pass.draw(0..render_object.vertex_count, 0..1);
            }

            for text_object in self.text_objects.values() {
                if !text_object.text.visible || text_object.text.opacity <= 0.0 {
                    continue;
                }

                render_pass.set_bind_group(0, &text_object.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &text_object.text_bind_group, &[]);
                render_pass.set_vertex_buffer(0, text_object.vertex_buffer.slice(..));
                render_pass.draw(0..text_object.vertex_count, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        self.queue.present(output);
    }

    pub fn create_text_object(&mut self, id: u32, text: Text) {
        let font = load_font();

        let (rgba_data, text_width, text_height) = create_text_bitmap_with_options(
            &font,
            &text.content,
            text.font_size,
            text.line_spacing,
            text.letter_spacing,
            text.max_width,
            text.alignment,
        );

        let text_texture = self.create_text_texture(&rgba_data, text_width, text_height);
        let text_view = text_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let text_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Text Sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let text_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Bind Group"),
            layout: &self.texture_bind_group_layout,
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

        let (vertex_buffer, vertex_count) = self.create_text_vertex_buffer(text_width, text_height);

        let uniforms = Self::text_uniforms(&text);

        let uniform_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Uniform Buffer"),
                contents: bytemuck::bytes_of(&uniforms),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Uniform Bind Group"),
            layout: &self.uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        self.text_objects.insert(
            id,
            TextObject {
                revision: text.revision(),
                text,
                uniform_buffer,
                uniform_bind_group,
                text_texture,
                text_bind_group,
                vertex_buffer,
                vertex_count,
            },
        );
    }

    fn create_text_texture(
        &self,
        rgba_data: &[u8],
        text_width: u32,
        text_height: u32,
    ) -> wgpu::Texture {
        let unpadded_bytes_per_row = text_width * 4;

        let padded_bytes_per_row = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
            * ((unpadded_bytes_per_row + wgpu::COPY_BYTES_PER_ROW_ALIGNMENT - 1)
                / wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);

        let mut padded_data = vec![0u8; (padded_bytes_per_row * text_height) as usize];

        for y in 0..text_height {
            let source_start = (y * unpadded_bytes_per_row) as usize;
            let source_end = source_start + unpadded_bytes_per_row as usize;
            let destination_start = (y * padded_bytes_per_row) as usize;
            let destination_end = destination_start + unpadded_bytes_per_row as usize;

            padded_data[destination_start..destination_end]
                .copy_from_slice(&rgba_data[source_start..source_end]);
        }

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
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

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
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

        texture
    }

    fn create_text_vertex_buffer(&self, text_width: u32, text_height: u32) -> (wgpu::Buffer, u32) {
        let screen_width = self.config.width.max(1) as f32;
        let screen_height = self.config.height.max(1) as f32;

        let text_width_ndc = (text_width as f32 / screen_width) * 2.0;
        let text_height_ndc = (text_height as f32 / screen_height) * 2.0;

        let half_width = text_width_ndc / 2.0;
        let half_height = text_height_ndc / 2.0;

        let vertices = Self::quad_vertices(half_width, half_height);

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        (vertex_buffer, vertices.len() as u32)
    }

    fn text_uniforms(text: &Text) -> Uniforms {
        Uniforms {
            position_rotation: [text.position[0], text.position[1], text.rotation, 0.0],
            scale: [text.scale[0], text.scale[1], 0.0, 0.0],
            color: [text.color[0], text.color[1], text.color[2], text.opacity],
            camera_position: [0.0, 0.0],
            camera_zoom: [1.0, 0.0],
        }
    }

    pub fn update_text_object(&mut self, id: u32, text: &Text) {
        let needs_bitmap_update = self
            .text_objects
            .get(&id)
            .map(|object| object.revision != text.revision())
            .unwrap_or(true);

        if needs_bitmap_update {
            let font = load_font();

            let (rgba_data, text_width, text_height) = create_text_bitmap_with_options(
                &font,
                &text.content,
                text.font_size,
                text.line_spacing,
                text.letter_spacing,
                text.max_width,
                text.alignment,
            );

            let text_texture = self.create_text_texture(&rgba_data, text_width, text_height);
            let text_view = text_texture.create_view(&wgpu::TextureViewDescriptor::default());

            let text_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("Text Sampler"),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                ..Default::default()
            });

            let text_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Text Bind Group"),
                layout: &self.texture_bind_group_layout,
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

            let (vertex_buffer, vertex_count) =
                self.create_text_vertex_buffer(text_width, text_height);

            if let Some(object) = self.text_objects.get_mut(&id) {
                object.text_texture = text_texture;
                object.text_bind_group = text_bind_group;
                object.vertex_buffer = vertex_buffer;
                object.vertex_count = vertex_count;
                object.revision = text.revision();
            }
        }

        if let Some(object) = self.text_objects.get_mut(&id) {
            let uniforms = Self::text_uniforms(text);

            self.queue
                .write_buffer(&object.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

            object.text = text.clone();
        }
    }
}
