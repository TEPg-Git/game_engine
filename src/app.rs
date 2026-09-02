use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::graphics::Uniforms;
use crate::renderer::Renderer;
use crate::state::GameState;

// ============================================================
// APP
// ============================================================

pub struct App {
    // WINDOW
    window: Option<Arc<Window>>,

    // RENDERER
    renderer: Option<Renderer>,

    // GAME STATE
    game_state: GameState,
}

// ============================================================
// APP
// ============================================================

impl App {
    pub fn new() -> Self {
        Self {
            window: None,

            renderer: None,

            game_state: GameState::new(),
        }
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

        let renderer = match &mut self.renderer {
            Some(renderer) => renderer,
            None => return,
        };

        renderer.config.width = width;
        renderer.config.height = height;

        renderer
            .surface
            .configure(&renderer.device, &renderer.config);

        println!("Resized to {}x{}", width, height);
    }

    // ========================================================
    // RENDER
    // ========================================================

    fn render(&mut self) {
        // ----------------------------------------------------
        // UPDATE GAME STATE
        // ----------------------------------------------------

        self.game_state.update();

        // ----------------------------------------------------
        // GET RENDERER
        // ----------------------------------------------------

        let renderer = match &self.renderer {
            Some(renderer) => renderer,
            None => return,
        };

        // ----------------------------------------------------
        // GPU OBJECTS
        // ----------------------------------------------------

        let surface = &renderer.surface;

        let device = &renderer.device;

        let queue = &renderer.queue;

        let uniform_buffer = &renderer.uniform_buffer;

        let uniform_bind_group = &renderer.uniform_bind_group;

        let text_bind_group = &renderer.text_bind_group;

        let text_vertex_buffer = &renderer.text_vertex_buffer;

        let sprite_bind_group = &renderer.sprite_bind_group;

        let sprite_vertex_buffer = &renderer.sprite_vertex_buffer;

        let render_pipeline = &renderer.render_pipeline;

        // ----------------------------------------------------
        // UPDATE UNIFORMS
        // ----------------------------------------------------

        let uniforms = Uniforms {
            position: self.game_state.position,

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
                surface.configure(&renderer.device, &renderer.config);

                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                surface.configure(&renderer.device, &renderer.config);

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
            label: Some("Render Encoder"),
        });

        // ====================================================
        // RENDER PASS
        // ====================================================

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

            // =================================================
            // PIPELINE
            // =================================================

            render_pass.set_pipeline(render_pipeline);

            // =================================================
            // UNIFORMS
            // =================================================

            render_pass.set_bind_group(0, uniform_bind_group, &[]);

            // =================================================
            // SPRITE
            // =================================================

            render_pass.set_bind_group(1, sprite_bind_group, &[]);

            render_pass.set_vertex_buffer(0, sprite_vertex_buffer.slice(..));

            render_pass.draw(0..renderer.sprite_vertex_count, 0..1);

            // =================================================
            // TEXT
            // =================================================

            render_pass.set_bind_group(1, text_bind_group, &[]);

            render_pass.set_vertex_buffer(0, text_vertex_buffer.slice(..));

            render_pass.draw(0..renderer.text_vertex_count, 0..1);
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
        // RENDERER
        // ====================================================

        let renderer = Renderer::new(window.clone());

        // ====================================================
        // STORE
        // ====================================================

        self.window = Some(window);

        self.renderer = Some(renderer);

        // ====================================================
        // STARTUP
        // ====================================================

        println!("================================");

        println!("East Engine v0.4 initialized!");

        println!("Texture rendering enabled");

        println!("Sprite rendering enabled");

        println!("Text rendering enabled");

        println!("Text: Hello East Engine");

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
                        // ------------------------------------
                        // GAME INPUT
                        // ------------------------------------

                        self.game_state.keyboard.handle_keyboard(key_code, pressed);
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
