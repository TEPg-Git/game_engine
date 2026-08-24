// ============================================================
// GAME ENGINE v0.2
// WGPU + WINIT
//
// Features:
// - WASD movement
// - I/O speed control
// - Triangle rendering
// - PNG/JPG texture rendering
// ============================================================

use std::sync::Arc;

use wgpu::util::DeviceExt;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

// ============================================================
// TEXTURE MODULE
// ============================================================

mod texture;

use texture::Texture;

// ============================================================
// VERTEX
// ============================================================

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],

    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2
    ];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,

            step_mode: wgpu::VertexStepMode::Vertex,

            attributes: &Self::ATTRIBS,
        }
    }
}

// ============================================================
// UNIFORMS
// ============================================================

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    position: [f32; 2],
}

// ============================================================
// KEYBOARD STATE
// ============================================================

#[derive(Default)]
struct KeyboardState {
    w: bool,
    s: bool,
    a: bool,
    d: bool,

    i: bool,
    o: bool,
}

// ============================================================
// APP
// ============================================================

struct App {
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
    // GPU BUFFERS
    // --------------------------------------------------------
    vertex_buffer: Option<wgpu::Buffer>,

    uniform_buffer: Option<wgpu::Buffer>,

    // --------------------------------------------------------
    // BIND GROUPS
    // --------------------------------------------------------
    uniform_bind_group: Option<wgpu::BindGroup>,

    texture_bind_group: Option<wgpu::BindGroup>,

    // --------------------------------------------------------
    // TEXTURE
    // --------------------------------------------------------
    texture: Option<Texture>,

    // --------------------------------------------------------
    // RENDER PIPELINE
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
// APP IMPLEMENTATION
// ============================================================

impl App {
    fn new() -> Self {
        Self {
            window: None,

            surface: None,

            device: None,

            queue: None,

            config: None,

            vertex_buffer: None,

            uniform_buffer: None,

            uniform_bind_group: None,

            texture_bind_group: None,

            texture: None,

            render_pipeline: None,

            keyboard: KeyboardState::default(),

            position: [0.0, 0.0],

            speed: 0.001,
        }
    }

    // ========================================================
    // KEYBOARD INPUT
    // ========================================================

    fn handle_keyboard(&mut self, key_code: KeyCode, pressed: bool) {
        match key_code {
            KeyCode::KeyW => {
                self.keyboard.w = pressed;
            }

            KeyCode::KeyS => {
                self.keyboard.s = pressed;
            }

            KeyCode::KeyA => {
                self.keyboard.a = pressed;
            }

            KeyCode::KeyD => {
                self.keyboard.d = pressed;
            }

            KeyCode::KeyI => {
                self.keyboard.i = pressed;
            }

            KeyCode::KeyO => {
                self.keyboard.o = pressed;
            }

            _ => {}
        }
    }

    // ========================================================
    // UPDATE
    // ========================================================

    fn update(&mut self) {
        // ----------------------------------------------------
        // MOVE UP
        // ----------------------------------------------------

        if self.keyboard.w {
            self.position[1] += self.speed;
        }

        // ----------------------------------------------------
        // MOVE DOWN
        // ----------------------------------------------------

        if self.keyboard.s {
            self.position[1] -= self.speed;
        }

        // ----------------------------------------------------
        // MOVE LEFT
        // ----------------------------------------------------

        if self.keyboard.a {
            self.position[0] -= self.speed;
        }

        // ----------------------------------------------------
        // MOVE RIGHT
        // ----------------------------------------------------

        if self.keyboard.d {
            self.position[0] += self.speed;
        }

        // ----------------------------------------------------
        // INCREASE SPEED
        // ----------------------------------------------------

        if self.keyboard.i {
            self.speed += 0.001;
        }

        // ----------------------------------------------------
        // DECREASE SPEED
        // ----------------------------------------------------

        if self.keyboard.o {
            self.speed -= 0.001;
        }

        // ----------------------------------------------------
        // PREVENT NEGATIVE SPEED
        // ----------------------------------------------------

        self.speed = self.speed.max(0.0001);

        // ----------------------------------------------------
        // KEEP OBJECT INSIDE SCREEN
        // ----------------------------------------------------

        self.position[0] = self.position[0].clamp(-1.0, 1.0);

        self.position[1] = self.position[1].clamp(-1.0, 1.0);
    }

    fn toggle_fullscreen(&mut self) {
        if let Some(window) = &self.window {
            if window.fullscreen().is_some() {
                // --------------------------------------------
                // RETURN TO WINDOWED MODE
                // --------------------------------------------

                window.set_fullscreen(None);

                println!("Fullscreen: OFF");
            } else {
                // --------------------------------------------
                // ENTER FULLSCREEN
                // --------------------------------------------

                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

                println!("Fullscreen: ON");
            }
        }
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
        // GET WGPU OBJECTS
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

        let vertex_buffer = match &self.vertex_buffer {
            Some(buffer) => buffer,
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

        let texture_bind_group = match &self.texture_bind_group {
            Some(bind_group) => bind_group,
            None => return,
        };

        let render_pipeline = match &self.render_pipeline {
            Some(pipeline) => pipeline,
            None => return,
        };

        // ----------------------------------------------------
        // UPDATE UNIFORM
        // ----------------------------------------------------

        let uniforms = Uniforms {
            position: self.position,
        };

        queue.write_buffer(uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        // ----------------------------------------------------
        // GET FRAME
        // ----------------------------------------------------

        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,

            _ => {
                return;
            }
        };

        // ----------------------------------------------------
        // CREATE VIEW
        // ----------------------------------------------------

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // ----------------------------------------------------
        // COMMAND ENCODER
        // ----------------------------------------------------

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // ----------------------------------------------------
        // RENDER PASS
        // ----------------------------------------------------

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),

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

            render_pass.set_bind_group(1, texture_bind_group, &[]);

            // ------------------------------------------------
            // VERTICES
            // ------------------------------------------------

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

            // ------------------------------------------------
            // DRAW
            // ------------------------------------------------

            render_pass.draw(0..6, 0..1);
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
}

// ============================================================
// APPLICATION HANDLER
// ============================================================

impl ApplicationHandler for App {
    // ========================================================
    // RESUMED
    // ========================================================

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // ----------------------------------------------------
        // WINDOW
        // ----------------------------------------------------

        let window_attributes = Window::default_attributes().with_title("My Game Engine v0.2");

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // ----------------------------------------------------
        // WGPU INSTANCE
        // ----------------------------------------------------

        let instance = wgpu::Instance::default();

        // ----------------------------------------------------
        // SURFACE
        // ----------------------------------------------------

        let surface = instance.create_surface(window.clone()).unwrap();

        // ----------------------------------------------------
        // ADAPTER
        // ----------------------------------------------------

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),

            force_fallback_adapter: false,

            compatible_surface: Some(&surface),

            apply_limit_buckets: false,
        }))
        .expect("Failed to find a suitable GPU adapter");

        println!("GPU: {:?}", adapter.get_info());

        // ----------------------------------------------------
        // DEVICE + QUEUE
        // ----------------------------------------------------

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Game Engine Device"),

            required_features: wgpu::Features::empty(),

            required_limits: wgpu::Limits::default(),

            experimental_features: wgpu::ExperimentalFeatures::disabled(),

            memory_hints: wgpu::MemoryHints::default(),

            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create GPU device");

        println!("Device created!");

        // ----------------------------------------------------
        // SURFACE CONFIGURATION
        // ----------------------------------------------------

        let size = window.inner_size();

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface is not supported");

        // ----------------------------------------------------
        // VSYNC
        // ----------------------------------------------------

        config.present_mode = wgpu::PresentMode::Fifo;

        surface.configure(&device, &config);

        println!("Surface configured!");

        // ====================================================
        // TEXTURED QUAD
        // ====================================================

        let vertices = [
            // ------------------------------------------------
            // TOP LEFT
            // ------------------------------------------------
            Vertex {
                position: [-0.5, 0.5],

                tex_coords: [0.0, 0.0],
            },
            // ------------------------------------------------
            // TOP RIGHT
            // ------------------------------------------------
            Vertex {
                position: [0.5, 0.5],

                tex_coords: [1.0, 0.0],
            },
            // ------------------------------------------------
            // BOTTOM RIGHT
            // ------------------------------------------------
            Vertex {
                position: [0.5, -0.5],

                tex_coords: [1.0, 1.0],
            },
            // ------------------------------------------------
            // TOP LEFT
            // ------------------------------------------------
            Vertex {
                position: [-0.5, 0.5],

                tex_coords: [0.0, 0.0],
            },
            // ------------------------------------------------
            // BOTTOM RIGHT
            // ------------------------------------------------
            Vertex {
                position: [0.5, -0.5],

                tex_coords: [1.0, 1.0],
            },
            // ------------------------------------------------
            // BOTTOM LEFT
            // ------------------------------------------------
            Vertex {
                position: [-0.5, -0.5],

                tex_coords: [0.0, 1.0],
            },
        ];

        // ----------------------------------------------------
        // VERTEX BUFFER
        // ----------------------------------------------------

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Textured Quad Vertex Buffer"),

            contents: bytemuck::cast_slice(&vertices),

            usage: wgpu::BufferUsages::VERTEX,
        });

        println!("Vertex buffer created!");

        // ====================================================
        // UNIFORMS
        // ====================================================

        let uniforms = Uniforms {
            position: [0.0, 0.0],
        };

        // ----------------------------------------------------
        // UNIFORM BUFFER
        // ----------------------------------------------------

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

                    visibility: wgpu::ShaderStages::VERTEX,

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
        // LOAD IMAGE
        // ====================================================

        let texture = Texture::from_file(&device, &queue, "assets/test.jpg");

        println!("Texture loaded!");

        // ====================================================
        // TEXTURE BIND GROUP LAYOUT
        // ====================================================

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),

                entries: &[
                    // ------------------------------------
                    // TEXTURE
                    // ------------------------------------
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
                    // ------------------------------------
                    // SAMPLER
                    // ------------------------------------
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,

                        visibility: wgpu::ShaderStages::FRAGMENT,

                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),

                        count: None,
                    },
                ],
            });

        // ====================================================
        // TEXTURE BIND GROUP
        // ====================================================

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),

            layout: &texture_bind_group_layout,

            entries: &[
                // ------------------------------------
                // TEXTURE
                // ------------------------------------
                wgpu::BindGroupEntry {
                    binding: 0,

                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                // ------------------------------------
                // SAMPLER
                // ------------------------------------
                wgpu::BindGroupEntry {
                    binding: 1,

                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        println!("Texture bind group created!");

        // ====================================================
        // SHADER
        // ====================================================

        let shader_source = r#"

            // =================================================
            // VERTEX INPUT
            // =================================================

            struct VertexInput {

                @location(0)
                position: vec2<f32>,

                @location(1)
                tex_coords: vec2<f32>,
            };

            // =================================================
            // VERTEX OUTPUT
            // =================================================

            struct VertexOutput {

                @builtin(position)
                position: vec4<f32>,

                @location(0)
                tex_coords: vec2<f32>,
            };

            // =================================================
            // UNIFORMS
            // =================================================

            struct Uniforms {

                position: vec2<f32>,
            };

            @group(0)
            @binding(0)
            var<uniform> uniforms: Uniforms;

            // =================================================
            // TEXTURE
            // =================================================

            @group(1)
            @binding(0)
            var texture_data: texture_2d<f32>;

            // =================================================
            // SAMPLER
            // =================================================

            @group(1)
            @binding(1)
            var texture_sampler: sampler;

            // =================================================
            // VERTEX SHADER
            // =================================================

            @vertex
            fn vs_main(
                input: VertexInput
            ) -> VertexOutput {

                var output: VertexOutput;

                output.position =
                    vec4<f32>(
                        input.position.x
                            + uniforms.position.x,

                        input.position.y
                            + uniforms.position.y,

                        0.0,

                        1.0
                    );

                output.tex_coords =
                    input.tex_coords;

                return output;
            }

            // =================================================
            // FRAGMENT SHADER
            // =================================================

            @fragment
            fn fs_main(
                input: VertexOutput
            ) -> @location(0) vec4<f32> {

                return textureSample(
                    texture_data,
                    texture_sampler,
                    input.tex_coords
                );
            }

        "#;

        // ----------------------------------------------------
        // SHADER MODULE
        // ----------------------------------------------------

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Texture Shader"),

            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        println!("Shader created!");

        // ====================================================
        // PIPELINE LAYOUT
        // ====================================================

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Texture Pipeline Layout"),

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
            label: Some("Texture Render Pipeline"),

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
        // STORE EVERYTHING
        // ====================================================

        self.window = Some(window);

        self.surface = Some(surface);

        self.device = Some(device);

        self.queue = Some(queue);

        self.config = Some(config);

        self.vertex_buffer = Some(vertex_buffer);

        self.uniform_buffer = Some(uniform_buffer);

        self.uniform_bind_group = Some(uniform_bind_group);

        self.texture_bind_group = Some(texture_bind_group);

        self.texture = Some(texture);

        self.render_pipeline = Some(render_pipeline);

        println!("================================");

        println!("Game Engine v0.2 initialized!");

        println!("WASD = Move");

        println!("I/O = Change speed");

        println!("PNG/JPG texture loaded");

        println!("================================");

        // ----------------------------------------------------
        // FIRST FRAME
        // ----------------------------------------------------

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
                    // --------------------------------------------
                    // F = TOGGLE FULLSCREEN
                    // --------------------------------------------

                    if key_code == KeyCode::KeyF && pressed {
                        self.toggle_fullscreen();
                    } else {
                        self.handle_keyboard(key_code, pressed);
                    }
                }
            }

            // ------------------------------------------------
            // DRAW
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

// ============================================================
// MAIN
// ============================================================

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let mut app = App::new();

    event_loop.run_app(&mut app).expect("Event loop failed");
}
