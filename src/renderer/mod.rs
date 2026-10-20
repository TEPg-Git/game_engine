use std::collections::HashMap;
use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::entity::Entity;
use crate::graphics::{CameraUniforms, Uniforms, Vertex};
use crate::sprite::Sprite;
use crate::state::GameState;
use crate::text::Text;

mod particle_renderer;
mod resources;
mod text_renderer;

pub use self::particle_renderer::ParticleInstance;

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
// RENDER OBJECT
// ============================================================

pub struct RenderObject {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub sprite_bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub transform_revision: u64,
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

    // RENDER DIAGNOSTICS
    pub draw_calls: u32,
    pub buffer_writes: u32,

    // SHARED GPU LAYOUTS
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,

    // PIPELINE
    pub render_pipeline: wgpu::RenderPipeline,
    pub particle_pipeline: wgpu::RenderPipeline,
    pub particle_vertex_buffer: wgpu::Buffer,
    pub particle_vertex_count: u32,
    pub particle_instance_buffer: wgpu::Buffer,
    particle_instance_capacity: usize,
    particle_uniform_buffer: wgpu::Buffer,
    particle_uniform_bind_group: wgpu::BindGroup,
    particle_bind_group: Option<wgpu::BindGroup>,
    pub text_sampler: wgpu::Sampler,
    pub camera_uniform_buffer: wgpu::Buffer,

    // PENDING SURFACE SIZE
    pending_width: u32,
    pending_height: u32,
    resize_pending: bool,

    // RENDER OBJECTS
    pub render_objects: Vec<Option<RenderObject>>,
    pub text_objects: HashMap<u32, TextObject>,
    last_camera: CameraUniforms,
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
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
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

        let particle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Particle Instanced Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_particle"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Some(Vertex::layout()), Some(ParticleInstance::layout())],
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

        let text_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Text Sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let camera_uniforms = CameraUniforms {
            position: [0.0, 0.0],
            zoom: 1.0,
            _padding: 0.0,
        };

        let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::bytes_of(&camera_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        const PARTICLE_BUFFER_CAPACITY: usize = 1000;

        let particle_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Particle Instance Buffer"),
            size: (std::mem::size_of::<ParticleInstance>() * PARTICLE_BUFFER_CAPACITY)
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Match the current particle sprite size (0.01 x 0.01 world units).
        // Each instance supplies only its center position.
        let particle_vertices = Self::quad_vertices(0.0015, 0.0015);
        let particle_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shared Particle Vertex Buffer"),
            contents: bytemuck::cast_slice(&particle_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let particle_vertex_count = particle_vertices.len() as u32;

        let particle_uniforms = Uniforms {
            position_rotation: [0.0, 0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        };
        let particle_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Particle Uniform Buffer"),
                contents: bytemuck::bytes_of(&particle_uniforms),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let particle_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: camera_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            surface,
            device,
            queue,
            draw_calls: 0,
            buffer_writes: 0,
            pending_width: config.width,
            pending_height: config.height,
            resize_pending: false,
            config,
            texture_bind_group_layout,
            uniform_bind_group_layout,
            render_pipeline,
            particle_pipeline,
            particle_vertex_buffer,
            particle_vertex_count,
            particle_instance_buffer,
            particle_instance_capacity: PARTICLE_BUFFER_CAPACITY,
            particle_uniform_buffer,
            particle_uniform_bind_group,
            particle_bind_group: None,
            text_sampler,
            camera_uniform_buffer,
            render_objects: Vec::with_capacity(16),
            text_objects: HashMap::new(),
            last_camera: camera_uniforms,
        }
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
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.camera_uniform_buffer.as_entire_binding(),
                    },
                ],
            });

            let sprite_bind_group = self.create_sprite_bind_group(sprite);
            let (vertex_buffer, vertex_count) = self.create_sprite_vertex_buffer(sprite);

            let id = entity.id as usize;
            if self.render_objects.len() <= id {
                self.render_objects.resize_with(id + 1, || None);
            }

            self.render_objects[id] = Some(RenderObject {
                uniform_buffer,
                uniform_bind_group,
                sprite_bind_group,
                vertex_buffer,
                vertex_count,
                transform_revision: entity.transform.revision(),
            });
        }
    }
    pub fn initialize_particle_resources(&mut self, sprite: &Sprite) {
        self.particle_bind_group = Some(self.create_sprite_bind_group(sprite));
    }
    pub fn render(&mut self, state: &GameState, fps_text: &Text, frame_time_text: &Text) {
        if self.config.width == 0 || self.config.height == 0 {
            return;
        }

        self.draw_calls = 0;
        self.buffer_writes = 0;

        let particle_instances: Vec<ParticleInstance> = state
            .particle_system
            .particles
            .iter()
            .map(|particle| {
                ParticleInstance::from_position_color(particle.position, particle.color)
            })
            .collect();

        // Grow the instance buffer if the particle count is increased at runtime.
        if particle_instances.len() > self.particle_instance_capacity {
            self.particle_instance_capacity = particle_instances.len().next_power_of_two();
            self.particle_instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Particle Instance Buffer"),
                size: (std::mem::size_of::<ParticleInstance>() * self.particle_instance_capacity)
                    as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        if !particle_instances.is_empty() {
            self.queue.write_buffer(
                &self.particle_instance_buffer,
                0,
                bytemuck::cast_slice(&particle_instances),
            );
            self.buffer_writes += 1;
        }

        self.update_text_object(0, &state.text);
        self.update_text_object(1, &state.score_text);
        self.update_text_object(2, fps_text);
        self.update_text_object(3, frame_time_text);

        let camera_uniforms = CameraUniforms {
            position: [state.camera.position[0], state.camera.position[1]],
            zoom: state.camera.zoom,
            _padding: 0.0,
        };

        if camera_uniforms.position != self.last_camera.position
            || camera_uniforms.zoom != self.last_camera.zoom
        {
            self.queue.write_buffer(
                &self.camera_uniform_buffer,
                0,
                bytemuck::bytes_of(&camera_uniforms),
            );
            self.buffer_writes += 1;
            self.last_camera = camera_uniforms;
        }

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

            // Draw every particle from one shared quad + one per-instance position buffer.
            if !particle_instances.is_empty() {
                if let Some(particle_bind_group) = self.particle_bind_group.as_ref() {
                    render_pass.set_pipeline(&self.particle_pipeline);
                    render_pass.set_bind_group(0, &self.particle_uniform_bind_group, &[]);
                    render_pass.set_bind_group(1, particle_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, self.particle_vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, self.particle_instance_buffer.slice(..));
                    render_pass.draw(
                        0..self.particle_vertex_count,
                        0..particle_instances.len() as u32,
                    );
                    self.draw_calls += 1;
                }
            }

            render_pass.set_pipeline(&self.render_pipeline);

            for entity in &state.entities {
                let Some(render_object) = self
                    .render_objects
                    .get_mut(entity.id as usize)
                    .and_then(Option::as_mut)
                else {
                    continue;
                };

                if render_object.transform_revision != entity.transform.revision() {
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
                    };

                    self.queue.write_buffer(
                        &render_object.uniform_buffer,
                        0,
                        bytemuck::bytes_of(&uniforms),
                    );
                    self.buffer_writes += 1;

                    render_object.transform_revision = entity.transform.revision();
                }

                render_pass.set_bind_group(0, &render_object.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &render_object.sprite_bind_group, &[]);
                render_pass.set_vertex_buffer(0, render_object.vertex_buffer.slice(..));
                render_pass.draw(0..render_object.vertex_count, 0..1);
                self.draw_calls += 1;
            }

            for text_object in self.text_objects.values() {
                if !text_object.text.visible || text_object.text.opacity <= 0.0 {
                    continue;
                }

                render_pass.set_bind_group(0, &text_object.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &text_object.text_bind_group, &[]);
                render_pass.set_vertex_buffer(0, text_object.vertex_buffer.slice(..));
                render_pass.draw(0..text_object.vertex_count, 0..1);
                self.draw_calls += 1;
            }
        }

        self.queue.submit(Some(encoder.finish()));
        self.queue.present(output);
    }

}
