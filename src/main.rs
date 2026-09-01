mod app;
mod graphics;
mod input;
mod renderer;
mod state;
mod text;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let mut app = App::new();

    event_loop.run_app(&mut app).expect("Event loop failed");
}
