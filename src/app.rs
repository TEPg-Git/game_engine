use crate::renderer::Renderer;
use crate::sprite::Sprite;
use crate::state::GameState;
use crate::time::Time;
use std::sync::Arc;
use std::time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

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

    // TIME
    time: Time,

    // RESIZE
    // Windows can emit a stream of resize events while the user is dragging.
    // Wait briefly after the last event before requesting a frame.
    resize_pending: bool,
    last_resize: Option<Instant>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            game_state: GameState::new(),
            time: Time::new(),
            resize_pending: false,
            last_resize: None,
        }
    }

    fn toggle_fullscreen(&self) {
        let Some(window) = &self.window else {
            return;
        };

        if window.fullscreen().is_some() {
            window.set_fullscreen(None);
        } else {
            window.set_fullscreen(Some(
                winit::window::Fullscreen::Borderless(None),
            ));
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        renderer.resize(width, height);
    }

    fn update(&mut self) {
        self.time.update();
        self.game_state.update(self.time.delta_time());
    }

    fn render(&mut self) {
        self.update();

        let Some(renderer) = &mut self.renderer else {
            return;
        };

        renderer.render(&self.game_state);
    }

    fn initialize_scene(&mut self, renderer: &mut Renderer) {
        let player_sprite_size = [0.05, 0.4];
        let ball_sprite_size = [0.2, 0.2];

        let player_sprite_1 =
            Sprite::from_file(&renderer.device, &renderer.queue, "assets/textures/Player.png", player_sprite_size);
        let player_sprite_2 =
            Sprite::from_file(&renderer.device, &renderer.queue, "assets/textures/Player.png", player_sprite_size);
        let ball_sprite =
            Sprite::from_file(&renderer.device, &renderer.queue, "assets/textures/Ball.png", ball_sprite_size);

        let player1_id = self.game_state.player1_id;
        let player2_id = self.game_state.player2_id;
        let ball_id = self.game_state.ball_id;

        if let Some(entity) = self.game_state.get_entity_mut(player1_id) {
            entity.set_sprite(player_sprite_1);
        }

        if let Some(entity) = self.game_state.get_entity_mut(player2_id) {
            entity.set_sprite(player_sprite_2);
        }

        if let Some(entity) = self.game_state.get_entity_mut(ball_id) {
            entity.set_sprite(ball_sprite);
        }

        for entity in &self.game_state.entities {
            renderer.create_render_object(entity);
        }

        renderer.create_text_object(0, self.game_state.text.clone());
        renderer.create_text_object(1, self.game_state.score_text.clone());

    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("East Engine");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let mut renderer = Renderer::new(window.clone());
        self.initialize_scene(&mut renderer);

        self.window = Some(window);
        self.renderer = Some(renderer);

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
                self.resize_pending = true;
                self.last_resize = Some(Instant::now());
            }

            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;

                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if key_code == KeyCode::Escape && pressed {
                        event_loop.exit();
                        return;
                    }

                    if key_code == KeyCode::KeyF && pressed {
                        self.toggle_fullscreen();
                    } else {
                        self.game_state.keyboard.handle_keyboard(key_code, pressed);
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                self.render();

                // Do not continuously request redraws while the window is
                // being resized. The OS can generate a large number of
                // resize events during a drag, and rendering during that
                // interaction can stall the UI thread.
                if !self.resize_pending {
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if !self.resize_pending {
            return;
        }

        let Some(last_resize) = self.last_resize else {
            return;
        };

        if last_resize.elapsed() >= Duration::from_millis(100) {
            self.resize_pending = false;
            self.last_resize = None;

            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}
