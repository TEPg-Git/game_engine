use crate::text::{Text, TextAlignment};
use crate::time::Time;
use std::time::Duration;

const DISPLAY_INTERVAL: Duration = Duration::from_millis(250);

pub struct Diagnostics {
    fps: f32,
    frame_time_ms: f32,
    pub fps_text: Text,
    pub frame_time_ms_text: Text,
    display_timer: Duration,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            frame_time_ms: 0.0,
            display_timer: Duration::ZERO,
            fps_text: {
                let mut text = Text::new("FPS: 0", 24.0);
                text.set_position(-0.9, 0.9);
                text.set_alignment(TextAlignment::Left);
                text
            },
            frame_time_ms_text: {
                let mut text = Text::new("Frame: 0.00 ms", 24.0);
                text.set_position(-0.9, 0.8);
                text.set_alignment(TextAlignment::Left);
                text
            },
        }
    }

    pub fn update(&mut self, time: &Time) {
        self.fps = time.fps();
        self.frame_time_ms = time.raw_delta_time() * 1000.0;
        self.display_timer += Duration::from_secs_f32(time.raw_delta_time());

        if self.display_timer < DISPLAY_INTERVAL {
            return;
        }

        self.display_timer -= DISPLAY_INTERVAL;

        let fps_text = format!("FPS: {:.0}", self.fps);
        let frame_text = format!("Frame: {:.2} ms", self.frame_time_ms);

        self.fps_text.set_content(&fps_text);
        self.frame_time_ms_text.set_content(&frame_text);
    }
}