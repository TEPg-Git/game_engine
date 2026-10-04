use crate::camera::Camera;
use crate::entity::Entity;
use crate::particle::Particle;
use crate::text::Text;

pub struct GameState {
    pub camera: Camera,
    pub entities: Vec<Entity>,
    pub text: Text,
    pub score_text: Text,
    pub particle: Particle,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            entities: Vec::new(),
            text: Text::new("", 16.0),
            score_text: Text::new("", 16.0),
            particle: Particle::new([0.0, 0.0], [1.0, 0.5], 1.0),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.particle.update(delta_time);

        if let Some(entity) = self.entities.get_mut(0) {
            entity
                .transform
                .set_position(self.particle.position[0], self.particle.position[1]);
        }
    }
}
