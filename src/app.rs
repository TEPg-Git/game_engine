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
    //The main application struct.
    // WINDOW
    window: Option<Arc<Window>>, //Actual Application Window ARC-Atomically Reference Counted,allows shared ownership

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
        //Creates the initial state of the application
        Self {
            window: None,

            renderer: None,

            game_state: GameState::new(), //GameState doesn't need the OS window or GPU to exist yet

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
            //If a window currently exists, give me access to it.
            if window.fullscreen().is_some() {
                //Are we currently fullscreen?
                window.set_fullscreen(None); //If so, exit fullscreen mode.

                println!("Fullscreen: OFF");
            } else {
                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None))); //If not, enter fullscreen mode.

                println!("Fullscreen: ON");
            }
        }
    }

    // ========================================================
    // RESIZE
    // ========================================================

    fn resize(&mut self, width: u32, height: u32) {
        //Resizes the window to the given width and height.
        if width == 0 || height == 0 {
            //If either width or height is 0, ignore the request.
            return;
        }

        let renderer = match &mut self.renderer {
            //If the renderer exists, give me mutable access to it. Otherwise stop.
            Some(renderer) => renderer,
            None => return,
        };

        renderer.config.width = width; //Update the renderer's configuration with the new width and height.
        renderer.config.height = height;

        renderer
            .surface
            .configure(&renderer.device, &renderer.config); //"The window's rendering surface is now this size."

        println!("Resized to {}x{}", width, height); //Log the resize event.
    }

    // ========================================================
    // RENDER
    // ========================================================

    fn render(&mut self) {
        // ----------------------------------------------------
        // UPDATE GAME STATE
        // ----------------------------------------------------

        self.game_state.update(); //First update the game.

        // ----------------------------------------------------
        // GET RENDERER
        // ----------------------------------------------------

        let renderer = match self.renderer.as_mut() {
            //If the renderer exists, give me mutable access to it. Otherwise stop.
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
            //Here is where the text should be, how big it is, how it is rotated, and what color/opacity it has.
            position_rotation: [
                //Position and rotation of the text in screen space.
                self.game_state.text.position[0], //X position of the text.
                self.game_state.text.position[1], //Y position of the text.
                self.game_state.text.rotation,    //Rotation of the text.
                0.0,
            ],

            scale: [
                //Scale of the text.
                self.game_state.text.scale[0], //Horizontal scale of the text.
                self.game_state.text.scale[1], //Vertical scale of the text.
                0.0,
                0.0,
            ],

            color: [
                //Color and opacity of the text.
                self.game_state.text.color[0], //Red component of the text color.
                self.game_state.text.color[1], //Green component of the text color.
                self.game_state.text.color[2], //Blue component of the text color.
                self.game_state.text.opacity,  //Opacity of the text.
            ],

            camera_position: [0.0, 0.0], //Camera position is not used for text.
            camera_zoom: [1.0, 0.0],     //Camera zoom is not used for text.
        };

        renderer.queue.write_buffer(
            //Copy these CPU-side uniform values into the GPU buffer.
            &renderer.text_uniform_buffer, //The GPU buffer to write to.
            0,                             //Offset into the buffer to start writing.
            bytemuck::bytes_of(&text_uniforms), //The data to write.
        );

        // ----------------------------------------------------
        // GPU OBJECTS
        // ----------------------------------------------------

        let surface = &renderer.surface; //The surface to render to.
        let device = &renderer.device; //The GPU device.
        let queue = &renderer.queue; //The GPU queue.

        let uniform_buffer = &renderer.uniform_buffer; //The uniform buffer for the main render pipeline.
        let uniform_bind_group = &renderer.uniform_bind_group; //The bind group for the uniform buffer.

        let text_uniform_bind_group = &renderer.text_uniform_bind_group; //The bind group for the text uniform buffer.
        let text_bind_group = &renderer.text_bind_group; //The bind group for the text texture.
        let text_vertex_buffer = &renderer.text_vertex_buffer; //The vertex buffer for the text.

        let render_pipeline = &renderer.render_pipeline; //The render pipeline for the main render loop.

        // ====================================================
        // PLAYER TRANSFORM
        // ====================================================

        let player = match self.game_state.get_entity(0) {
            //Get the player entity from the game state.
            Some(player) => player, //Return the player entity if it exists.
            None => return,         //Return if the player entity does not exist.
        };

        // ====================================================
        // PLAYER UNIFORMS
        // ====================================================

        let uniforms = Uniforms {
            //Create the uniforms for the player sprite.
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

        queue.write_buffer(uniform_buffer, 0, bytemuck::bytes_of(&uniforms)); //Write the uniforms to the uniform buffer.

        // ----------------------------------------------------
        // GET FRAME
        // ----------------------------------------------------
        //Give me the next surface image that I can render into.
        let output = match surface.get_current_texture() {
            //Get the current texture from the surface.
            wgpu::CurrentSurfaceTexture::Success(output) => output, //Success means good

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => output, //Suboptimal means useable but not optimal

            wgpu::CurrentSurfaceTexture::Outdated => {
                //Outdated means the surface has been resized or lost
                surface.configure(&renderer.device, &renderer.config);
                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                //Lost means the surface has been lost
                surface.configure(&renderer.device, &renderer.config);
                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout => {
                //Timeout means the surface has not been rendered in time
                return;
            }

            wgpu::CurrentSurfaceTexture::Occluded => {
                //Occluded means the surface is covered by another window
                return;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                //Validation means the surface is not valid
                return;
            }
        };

        // ----------------------------------------------------
        // VIEW
        // ----------------------------------------------------
        //The texture is the underlying GPU image.
        //A view describes how you're going to access/use that texture.

        let view = output //Create a view of the texture to render into.
            .texture //Get the texture from the output.
            .create_view(&wgpu::TextureViewDescriptor::default()); //Default view descriptor is fine for our purposes.

        // ----------------------------------------------------
        // COMMAND ENCODER
        // ----------------------------------------------------
        //creates something that records GPU commands.

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            //Create a command encoder to record render commands.
            label: Some("Render Encoder"),
        });

        // ====================================================
        // RENDER PASS
        // ====================================================
        // "I'm going to perform rendering operations against this render target."

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                //Begin a render pass against the output texture.
                label: Some("Render Pass"),

                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,

                    depth_slice: None,

                    resolve_target: None,

                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            //Clear the color buffer with a dark blue color.
                            r: 0.05,
                            g: 0.10,
                            b: 0.20,
                            a: 1.0,
                        }),

                        store: wgpu::StoreOp::Store, //Keep the resulting rendered pixels.
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
            // The render pipeline defines how GPU rendering happens.

            render_pass.set_pipeline(render_pipeline); //Set the render pipeline for this render pass.

            // =================================================
            // SPRITE
            // =================================================
            // Set the bind group for the uniform buffer and player sprite.

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
                render_pass.set_bind_group(0, text_uniform_bind_group, &[]); //Set the uniform bind group for the text.

                render_pass.set_bind_group(1, text_bind_group, &[]); //Set the bind group for the text texture.

                render_pass.set_vertex_buffer(0, text_vertex_buffer.slice(..)); //Set the vertex buffer for the text.

                render_pass.draw(0..renderer.text_vertex_count, 0..1); //Draw these vertices once.
            }
        }

        // ----------------------------------------------------
        // SUBMIT
        // ----------------------------------------------------

        queue.submit(Some(encoder.finish())); //Submit the command encoder to the GPU queue.

        // ----------------------------------------------------
        // PRESENT
        // ----------------------------------------------------

        queue.present(output); //Present the output to the window.
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
