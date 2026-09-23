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
        self.delta_time = (current_frame - self.last_frame)
            .as_secs_f32()
            .min(0.1);

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
