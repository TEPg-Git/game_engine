use crate::time::Time;
pub struct Diagnostics {
    fps: f32,
    frame_time_ms: f32,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            frame_time_ms: 0.0,
        }
    }
    pub fn update(&mut self, time: &Time) {
        self.fps = time.fps();
        self.frame_time_ms = time.raw_delta_time() * 1000.0;
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn frame_time_ms(&self) -> f32 {
        self.frame_time_ms
    }
}
