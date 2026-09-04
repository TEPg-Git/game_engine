use crate::sprite::Sprite;
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

    // SPRITE
    pub player_sprite_bind_group: Option<wgpu::BindGroup>,
    pub player_sprite_vertex_buffer: Option<wgpu::Buffer>,
    pub player_sprite_vertex_count: u32,
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

            player_sprite_bind_group: None,

            player_sprite_vertex_buffer: None,

            player_sprite_vertex_count: 0,
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

        let renderer = match self.renderer.as_mut() {
            Some(renderer) => renderer,
            None => return,
        };

        // ----------------------------------------------------
        // UPDATE TEXT BITMAP
        // ----------------------------------------------------
        //
        // Content, font size, alignment, wrapping, line spacing
        // and letter spacing cause the text bitmap to change.
        //
        // Transform/color/opacity are handled through uniforms.
        // ----------------------------------------------------

        if renderer.text_revision != self.game_state.text.revision() {
            renderer.update_text(&self.game_state.text);
        }

        // ----------------------------------------------------
        // UPDATE TEXT UNIFORMS
        // ----------------------------------------------------
        //
        // Text is currently screen-space-like:
        // camera position = 0
        // camera zoom    = 1
        //
        // Therefore camera movement/zoom does not affect UI text.
        // ----------------------------------------------------

        let text_uniforms = Uniforms {
            position_rotation: [
                self.game_state.text.position[0],
                self.game_state.text.position[1],
                self.game_state.text.rotation,
                0.0,
            ],

            scale: [
                self.game_state.text.scale[0],
                self.game_state.text.scale[1],
                0.0,
                0.0,
            ],

            color: [
                self.game_state.text.color[0],
                self.game_state.text.color[1],
                self.game_state.text.color[2],
                self.game_state.text.opacity,
            ],

            camera_position: [0.0, 0.0],

            camera_zoom: [1.0, 0.0],
        };

        renderer.queue.write_buffer(
            &renderer.text_uniform_buffer,
            0,
            bytemuck::bytes_of(&text_uniforms),
        );

        // ----------------------------------------------------
        // GPU OBJECTS
        // ----------------------------------------------------

        let surface = &renderer.surface;
        let device = &renderer.device;
        let queue = &renderer.queue;

        let uniform_buffer = &renderer.uniform_buffer;
        let uniform_bind_group = &renderer.uniform_bind_group;

        let text_uniform_bind_group = &renderer.text_uniform_bind_group;
        let text_bind_group = &renderer.text_bind_group;
        let text_vertex_buffer = &renderer.text_vertex_buffer;

        let render_pipeline = &renderer.render_pipeline;

        // ====================================================
        // PLAYER TRANSFORM
        // ====================================================

        let player = match self.game_state.get_entity(0) {
            Some(player) => player,
            None => return,
        };

        // ====================================================
        // PLAYER UNIFORMS
        // ====================================================

        let uniforms = Uniforms {
            position_rotation: [
                player.transform.position[0],
                player.transform.position[1],
                player.transform.rotation,
                0.0,
            ],

            scale: [
                player.transform.scale[0],
                player.transform.scale[1],
                0.0,
                0.0,
            ],

            // Sprite texture must remain white so that the
            // original texture color is preserved.
            color: [1.0, 1.0, 1.0, 1.0],

            camera_position: [
                self.game_state.camera.position[0],
                self.game_state.camera.position[1],
            ],

            camera_zoom: [self.game_state.camera.zoom, 0.0],
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
            // SPRITE
            // =================================================

            render_pass.set_bind_group(0, uniform_bind_group, &[]);

            if let (Some(bind_group), Some(vertex_buffer)) = (
                self.player_sprite_bind_group.as_ref(),
                self.player_sprite_vertex_buffer.as_ref(),
            ) {
                render_pass.set_bind_group(1, bind_group, &[]);

                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                render_pass.draw(0..self.player_sprite_vertex_count, 0..1);
            }

            // =================================================
            // TEXT
            // =================================================
            //
            // Text uses its own uniform bind group so text
            // color/transform cannot affect the sprite.
            // =================================================

            if self.game_state.text.visible && self.game_state.text.opacity > 0.0 {
                render_pass.set_bind_group(0, text_uniform_bind_group, &[]);

                render_pass.set_bind_group(1, text_bind_group, &[]);

                render_pass.set_vertex_buffer(0, text_vertex_buffer.slice(..));

                render_pass.draw(0..renderer.text_vertex_count, 0..1);
            }
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

        let renderer = Renderer::new(window.clone(), &self.game_state.text);

        // ====================================================
        // PLAYER SPRITE
        // ====================================================

        let sprite = Sprite::from_file(
            &renderer.device,
            &renderer.queue,
            "assets/textures/Test.jpg",
            [0.5, 0.5],
        );

        if let Some(player) = self.game_state.get_entity_mut(0) {
            player.set_sprite(sprite);
        }

        // ====================================================
        // CREATE PLAYER GPU RESOURCES
        // ====================================================

        if let Some(player) = self.game_state.get_entity(0) {
            if let Some(sprite) = &player.sprite {
                let bind_group = renderer.create_sprite_bind_group(sprite);

                let (vertex_buffer, vertex_count) = renderer.create_sprite_vertex_buffer(sprite);

                self.player_sprite_bind_group = Some(bind_group);

                self.player_sprite_vertex_buffer = Some(vertex_buffer);

                self.player_sprite_vertex_count = vertex_count;
            }
        }

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
