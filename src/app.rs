use std::sync::Arc;

use wgpu::util::DeviceExt;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::graphics::{Uniforms, Vertex};
use crate::input::KeyboardState;
use crate::text::{create_text_bitmap, load_font};
// ============================================================
// APP
// ============================================================

pub struct App {
    // --------------------------------------------------------
    // WINDOW
    // --------------------------------------------------------
    window: Option<Arc<Window>>,

    // --------------------------------------------------------
    // WGPU
    // --------------------------------------------------------
    surface: Option<wgpu::Surface<'static>>,

    device: Option<wgpu::Device>,

    queue: Option<wgpu::Queue>,

    config: Option<wgpu::SurfaceConfiguration>,

    // --------------------------------------------------------
    // UNIFORM
    // --------------------------------------------------------
    uniform_buffer: Option<wgpu::Buffer>,

    uniform_bind_group: Option<wgpu::BindGroup>,

    // --------------------------------------------------------
    // TEXT
    // --------------------------------------------------------
    text_texture: Option<wgpu::Texture>,

    text_bind_group: Option<wgpu::BindGroup>,

    text_vertex_buffer: Option<wgpu::Buffer>,

    text_vertex_count: u32,

    // --------------------------------------------------------
    // PIPELINE
    // --------------------------------------------------------
    render_pipeline: Option<wgpu::RenderPipeline>,

    // --------------------------------------------------------
    // GAME STATE
    // --------------------------------------------------------
    keyboard: KeyboardState,

    position: [f32; 2],

    speed: f32,
}

// ============================================================
// APP
// ============================================================

impl App {
    pub fn new() -> Self {
        Self {
            // ------------------------------------------------
            // WINDOW
            // ------------------------------------------------
            window: None,

            // ------------------------------------------------
            // WGPU
            // ------------------------------------------------
            surface: None,

            device: None,

            queue: None,

            config: None,

            // ------------------------------------------------
            // UNIFORM
            // ------------------------------------------------
            uniform_buffer: None,

            uniform_bind_group: None,

            // ------------------------------------------------
            // TEXT
            // ------------------------------------------------
            text_texture: None,

            text_bind_group: None,

            text_vertex_buffer: None,

            text_vertex_count: 0,

            // ------------------------------------------------
            // PIPELINE
            // ------------------------------------------------
            render_pipeline: None,

            // ------------------------------------------------
            // GAME STATE
            // ------------------------------------------------
            keyboard: KeyboardState::default(),

            position: [0.0, 0.0],

            speed: 0.001,
        }
    }

    // ========================================================
    // UPDATE
    // ========================================================

    fn update(&mut self) {
        // ----------------------------------------------------
        // MOVEMENT
        // ----------------------------------------------------

        if self.keyboard.w {
            self.position[1] += self.speed;
        }

        if self.keyboard.s {
            self.position[1] -= self.speed;
        }

        if self.keyboard.a {
            self.position[0] -= self.speed;
        }

        if self.keyboard.d {
            self.position[0] += self.speed;
        }

        // ----------------------------------------------------
        // SPEED
        // ----------------------------------------------------

        if self.keyboard.i {
            self.speed += 0.0001;
        }

        if self.keyboard.o {
            self.speed -= 0.0001;
        }

        self.speed = self.speed.max(0.0001);

        // ----------------------------------------------------
        // SCREEN LIMIT
        // ----------------------------------------------------

        self.position[0] = self.position[0].clamp(-1.0, 1.0);

        self.position[1] = self.position[1].clamp(-1.0, 1.0);
    }

    // ========================================================
    // FULLSCREEN
    // ========================================================

    fn toggle_fullscreen(&mut self) {
        if let Some(window) = &self.window {
            if window.fullscreen().is_some() {
                window.set_fullscreen(None);

                println!("Fullscreen: OFF");
            } else {
                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

                println!("Fullscreen: ON");
            }
        }
    }

    // ========================================================
    // RESIZE
    // ========================================================

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        let surface = match &self.surface {
            Some(surface) => surface,
            None => return,
        };

        let device = match &self.device {
            Some(device) => device,
            None => return,
        };

        let config = match &mut self.config {
            Some(config) => config,
            None => return,
        };

        config.width = width;
        config.height = height;

        surface.configure(device, config);

        println!("Resized to {}x{}", width, height);
    }

    // ========================================================
    // RENDER
    // ========================================================

    fn render(&mut self) {
        // ----------------------------------------------------
        // UPDATE
        // ----------------------------------------------------

        self.update();

        // ----------------------------------------------------
        // GET GPU OBJECTS
        // ----------------------------------------------------

        let surface = match &self.surface {
            Some(surface) => surface,
            None => return,
        };

        let device = match &self.device {
            Some(device) => device,
            None => return,
        };

        let queue = match &self.queue {
            Some(queue) => queue,
            None => return,
        };

        let uniform_buffer = match &self.uniform_buffer {
            Some(buffer) => buffer,
            None => return,
        };

        let uniform_bind_group = match &self.uniform_bind_group {
            Some(bind_group) => bind_group,
            None => return,
        };

        let text_bind_group = match &self.text_bind_group {
            Some(bind_group) => bind_group,
            None => return,
        };

        let text_vertex_buffer = match &self.text_vertex_buffer {
            Some(buffer) => buffer,
            None => return,
        };

        let render_pipeline = match &self.render_pipeline {
            Some(pipeline) => pipeline,
            None => return,
        };

        // ----------------------------------------------------
        // UPDATE POSITION
        // ----------------------------------------------------

        let uniforms = Uniforms {
            position: self.position,
            _padding: [0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        };

        queue.write_buffer(uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        // ----------------------------------------------------
        // GET FRAME
        // ----------------------------------------------------

        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,

            wgpu::CurrentSurfaceTexture::Outdated => {
                if let (Some(device), Some(config)) = (&self.device, &self.config) {
                    surface.configure(device, config);
                }

                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                if let (Some(device), Some(config)) = (&self.device, &self.config) {
                    surface.configure(device, config);
                }

                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Occluded => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        // ----------------------------------------------------
        // VIEW
        // ----------------------------------------------------

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // ----------------------------------------------------
        // COMMAND ENCODER
        // ----------------------------------------------------

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Text Render Encoder"),
        });

        // ====================================================
        // RENDER PASS
        // ====================================================

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Text Render Pass"),

                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,

                    depth_slice: None,

                    resolve_target: None,

                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.10,
                            b: 0.20,
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

            // ------------------------------------------------
            // PIPELINE
            // ------------------------------------------------

            render_pass.set_pipeline(render_pipeline);

            // ------------------------------------------------
            // UNIFORM
            // ------------------------------------------------

            render_pass.set_bind_group(0, uniform_bind_group, &[]);

            // ------------------------------------------------
            // TEXTURE
            // ------------------------------------------------

            render_pass.set_bind_group(1, text_bind_group, &[]);

            // ------------------------------------------------
            // TEXT QUAD
            // ------------------------------------------------

            render_pass.set_vertex_buffer(0, text_vertex_buffer.slice(..));

            // ------------------------------------------------
            // DRAW
            // ------------------------------------------------

            render_pass.draw(0..self.text_vertex_count, 0..1);
        }

        // ----------------------------------------------------
        // SUBMIT
        // ----------------------------------------------------

        queue.submit(Some(encoder.finish()));

        // ----------------------------------------------------
        // PRESENT
        // ----------------------------------------------------

        queue.present(output);
    }
}

// ============================================================
// APPLICATION HANDLER
// ============================================================

impl ApplicationHandler for App {
    // ========================================================
    // RESUMED
    // ========================================================

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // ====================================================
        // WINDOW
        // ====================================================

        let window_attributes = Window::default_attributes().with_title("East Engine");

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

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
            position: [0.0, 0.0],
            _padding: [0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),

            contents: bytemuck::bytes_of(&uniforms),

            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        println!("Uniform buffer created!");

        // ====================================================
        // UNIFORM BIND GROUP
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

        // ----------------------------------------------------
        // PAD RGBA DATA
        // ----------------------------------------------------

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
                label: Some("Text Texture Bind Group Layout"),

                entries: &[
                    // --------------------------------
                    // TEXTURE
                    // --------------------------------
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
                    // --------------------------------
                    // SAMPLER
                    // --------------------------------
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

        // Convert pixels to NDC coordinates

        let text_width_ndc = (text_width as f32 / screen_width) * 2.0;

        let text_height_ndc = (text_height as f32 / screen_height) * 2.0;

        // Half dimensions

        let half_width = text_width_ndc / 2.0;

        let half_height = text_height_ndc / 2.0;

        // ====================================================
        // TEXT QUAD
        // ====================================================

        let text_vertices = [
            // TOP LEFT
            Vertex {
                position: [-half_width, half_height],

                tex_coords: [0.0, 0.0],
            },
            // TOP RIGHT
            Vertex {
                position: [half_width, half_height],

                tex_coords: [1.0, 0.0],
            },
            // BOTTOM LEFT
            Vertex {
                position: [-half_width, -half_height],

                tex_coords: [0.0, 1.0],
            },
            // TOP RIGHT
            Vertex {
                position: [half_width, half_height],

                tex_coords: [1.0, 0.0],
            },
            // BOTTOM RIGHT
            Vertex {
                position: [half_width, -half_height],

                tex_coords: [1.0, 1.0],
            },
            // BOTTOM LEFT
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
            label: Some("Text Shader"),

            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        println!("Shader created!");

        // ====================================================
        // PIPELINE LAYOUT
        // ====================================================

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),

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
            label: Some("Text Render Pipeline"),

            layout: Some(&pipeline_layout),

            // ----------------------------------------
            // VERTEX
            // ----------------------------------------
            vertex: wgpu::VertexState {
                module: &shader,

                entry_point: Some("vs_main"),

                compilation_options: wgpu::PipelineCompilationOptions::default(),

                buffers: &[Some(Vertex::layout())],
            },

            // ----------------------------------------
            // PRIMITIVE
            // ----------------------------------------
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,

                strip_index_format: None,

                front_face: wgpu::FrontFace::Ccw,

                cull_mode: None,

                unclipped_depth: false,

                polygon_mode: wgpu::PolygonMode::Fill,

                conservative: false,
            },

            // ----------------------------------------
            // DEPTH
            // ----------------------------------------
            depth_stencil: None,

            // ----------------------------------------
            // MULTISAMPLE
            // ----------------------------------------
            multisample: wgpu::MultisampleState {
                count: 1,

                mask: !0,

                alpha_to_coverage_enabled: false,
            },

            // ----------------------------------------
            // FRAGMENT
            // ----------------------------------------
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
        // STORE RESOURCES
        // ====================================================

        self.window = Some(window);

        self.surface = Some(surface);

        self.device = Some(device);

        self.queue = Some(queue);

        self.config = Some(config);

        self.uniform_buffer = Some(uniform_buffer);

        self.uniform_bind_group = Some(uniform_bind_group);

        self.text_texture = Some(text_texture);

        self.text_bind_group = Some(text_bind_group);

        self.text_vertex_buffer = Some(text_vertex_buffer);

        self.text_vertex_count = text_vertex_count;

        self.render_pipeline = Some(render_pipeline);

        // ====================================================
        // STARTUP
        // ====================================================

        println!("================================");

        println!("East Engine v0.3 initialized!");

        println!("Text rendering enabled");

        println!("Text: {}", text);

        println!("WASD = Move");

        println!("I/O = Change speed");

        println!("F = Fullscreen");

        println!("Escape = Exit");

        println!("================================");

        // ====================================================
        // REQUEST FIRST FRAME
        // ====================================================

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    // ========================================================
    // WINDOW EVENTS
    // ========================================================

    fn window_event(
        &mut self,

        event_loop: &ActiveEventLoop,

        _window_id: WindowId,

        event: WindowEvent,
    ) {
        match event {
            // ------------------------------------------------
            // CLOSE
            // ------------------------------------------------
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            // ------------------------------------------------
            // RESIZE
            // ------------------------------------------------
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }

            // ------------------------------------------------
            // KEYBOARD
            // ------------------------------------------------
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;

                if let PhysicalKey::Code(key_code) = event.physical_key {
                    // ----------------------------------------
                    // ESCAPE
                    // ----------------------------------------

                    if key_code == KeyCode::Escape {
                        if pressed {
                            println!("Escape pressed");

                            event_loop.exit();
                        }

                        return;
                    }

                    // ----------------------------------------
                    // F = FULLSCREEN
                    // ----------------------------------------

                    if key_code == KeyCode::KeyF && pressed {
                        self.toggle_fullscreen();
                    } else {
                        self.keyboard.handle_keyboard(key_code, pressed);
                    }
                }
            }

            // ------------------------------------------------
            // REDRAW
            // ------------------------------------------------
            WindowEvent::RedrawRequested => {
                self.render();

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            _ => {}
        }
    }
}
