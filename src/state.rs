use crate::particle::Particle;

pub struct GameState {
    pub particle: Particle,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            particle: Particle::new([0.0, 0.0], [1.0, 0.5], 1.0),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.particle.update(delta_time);
    }
}
