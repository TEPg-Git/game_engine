use std::time::Instant;

//========================================================
// TIME
// =======================================================

pub struct Time {
    last_frame: Instant,
    delta_time: f32,
    raw_delta_time: f32,
    fps_counter: FpsCounter,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            delta_time: 0.0,
            raw_delta_time: 0.0,
            fps_counter: FpsCounter::new(),
        }
    }

    // Updates the time elapsed since the last frame.
    pub fn update(&mut self) {
        let current_frame = Instant::now();

        // A resize/fullscreen transition can pause rendering for a long
        // time. Never feed that entire pause into gameplay physics.
        self.raw_delta_time = (current_frame - self.last_frame).as_secs_f32();
        self.delta_time = self.raw_delta_time.min(0.1);
        self.fps_counter.update(self.raw_delta_time);
        self.last_frame = current_frame;
    }

    // Reset the frame timer after a period where the game was not rendering.
    pub fn reset(&mut self) {
        self.last_frame = Instant::now();
        self.delta_time = 0.0;
        self.raw_delta_time = 0.0;
        self.fps_counter.reset();
    }

    // Returns the time elapsed since the last frame in seconds.
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn raw_delta_time(&self) -> f32 {
        self.raw_delta_time
    }

    pub fn fps(&self) -> f32 {
        self.fps_counter.fps()
    }
}

pub struct FpsCounter {
    frame_count: u64,
    elapsed_time: f32,
    fps: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            elapsed_time: 0.0,
            fps: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;
        self.elapsed_time += delta_time;

        if self.elapsed_time >= 1.0 {
            self.fps = self.frame_count as f32 / self.elapsed_time;

            self.frame_count = 0;
            self.elapsed_time -= 1.0;
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn reset(&mut self) {
        self.frame_count = 0;
        self.elapsed_time = 0.0;
        self.fps = 0.0;
    }
}
