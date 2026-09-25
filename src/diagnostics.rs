use crate::text::{Text, TextAlignment};
use crate::time::Time;
use std::time::Duration;

const DISPLAY_INTERVAL: Duration = Duration::from_millis(250);

pub struct Diagnostics {
    fps: f32,
    frame_time_ms: f32,
    fps_changed: bool,

    display_timer: Duration,

    pub fps_text: Text,
    pub frame_time_ms_text: Text,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            frame_time_ms: 0.0,
            fps_changed: false,
            display_timer: Duration::ZERO,

            fps_text: {
                let mut text = Text::new("FPS: 0", 24.0);

                text.set_position(-0.9, 0.9);
                text.set_color(1.0, 1.0, 1.0, 1.0);
                text.set_alignment(TextAlignment::Left);
                text.set_line_spacing(1.2);
                text.set_letter_spacing(0.5);
                text.set_scale(1.0, 1.0);
                text.set_rotation(0.0);
                text.set_opacity(1.0);
                text.set_visible(true);
                text.set_max_width(None);

                text
            },
            frame_time_ms_text: {
                let mut text = Text::new("Frame: 0.00 ms", 24.0);

                text.set_position(-0.9, 0.8);
                text.set_color(1.0, 1.0, 1.0, 1.0);
                text.set_alignment(TextAlignment::Left);
                text.set_line_spacing(1.2);
                text.set_letter_spacing(0.5);
                text.set_scale(1.0, 1.0);
                text.set_rotation(0.0);
                text.set_opacity(1.0);
                text.set_visible(true);
                text.set_max_width(None);

                text
            },
        }
    }

    pub fn update(&mut self, time: &Time) {
        self.fps = time.fps();
        self.frame_time_ms = time.raw_delta_time() * 1000.0;

        self.display_timer += Duration::from_secs_f32(time.raw_delta_time());

        if self.display_timer < DISPLAY_INTERVAL {
            self.fps_changed = false;
            return;
        }

        self.display_timer = Duration::ZERO;

        self.fps_changed = true;

        self.fps_text.set_content(&format!("FPS: {:.0}", self.fps));

        self.frame_time_ms_text
            .set_content(&format!("Frame: {:.2} ms", self.frame_time_ms));
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn frame_time_ms(&self) -> f32 {
        self.frame_time_ms
    }

    pub fn fps_changed(&self) -> bool {
        self.fps_changed
    }
}
