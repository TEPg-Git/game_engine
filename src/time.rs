use std::time::Duration;
use std::time::Instant;

//========================================================
// TIME
// =======================================================

pub struct Time {
    last_frame: Instant,
    delta_time: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            delta_time: 0.0,
        }
    }

    // Updates the time elapsed since the last frame.
    pub fn update(&mut self) {
        let current_frame = Instant::now();

        // A resize/fullscreen transition can pause rendering for a long
        // time. Never feed that entire pause into gameplay physics.
        self.delta_time = (current_frame - self.last_frame).as_secs_f32().min(0.1);

        self.last_frame = current_frame;
    }

    // Reset the frame timer after a period where the game was not rendering.
    pub fn reset(&mut self) {
        self.last_frame = Instant::now();
        self.delta_time = 0.0;
    }

    // Returns the time elapsed since the last frame in seconds.
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
}

struct FpsCounter {
    frame_count: u64,
    elapsed_time: Duration,
    fps: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            elapsed_time: Duration::ZERO,
            fps: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.frame_count += 1;
        self.elapsed_time += delta_time;

        if self.elapsed_time >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / self.elapsed_time.as_secs_f32();

            self.frame_count = 0;
            self.elapsed_time = Duration::ZERO;
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}
