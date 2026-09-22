use crate::sprite::Sprite;
use crate::time::Time;
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
    // The main application struct.

    // WINDOW
    window: Option<Arc<Window>>,

    // RENDERER
    renderer: Option<Renderer>,

    // GAME STATE
    game_state: GameState,

    time: Time,
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

            time: Time::new(),
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
        self.time.update();
        let delta_time = self.time.delta_time();
        self.game_state.update(delta_time);

        let renderer = match self.renderer.as_mut() {
            Some(renderer) => renderer,
            None => return,
        };

        renderer.update_text_object(0, &self.game_state.text);
        renderer.update_text_object(1, &self.game_state.score_text);

        let surface = &renderer.surface;
        let device = &renderer.device;
        let queue = &renderer.queue;
        let render_pipeline = &renderer.render_pipeline;

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
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return,
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                            r: 0.00,
                            g: 0.00,
                            b: 0.00,
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

            render_pass.set_pipeline(render_pipeline);

            // =================================================
            // ENTITY SPRITES
            // =================================================

            for entity in &self.game_state.entities {
                let Some(render_object) = renderer.render_objects.get(&entity.id) else {
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
                    camera_position: [
                        self.game_state.camera.position[0],
                        self.game_state.camera.position[1],
                    ],
                    camera_zoom: [self.game_state.camera.zoom, 0.0],
                };

                queue.write_buffer(
                    &render_object.uniform_buffer,
                    0,
                    bytemuck::bytes_of(&uniforms),
                );

                render_pass.set_bind_group(0, &render_object.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &render_object.sprite_bind_group, &[]);
                render_pass.set_vertex_buffer(0, render_object.vertex_buffer.slice(..));
                render_pass.draw(0..render_object.vertex_count, 0..1);
            }

            // =================================================
            // TEXT OBJECTS
            // =================================================

            for text_object in renderer.text_objects.values() {
                if !text_object.text.visible || text_object.text.opacity <= 0.0 {
                    continue;
                }

                render_pass.set_bind_group(0, &text_object.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &text_object.text_bind_group, &[]);
                render_pass.set_vertex_buffer(0, text_object.vertex_buffer.slice(..));
                render_pass.draw(0..text_object.vertex_count, 0..1);
            }
        }

        queue.submit(Some(encoder.finish()));
        queue.present(output);
    }

