use crate::camera::Camera;
use crate::entity::Entity;
use crate::particle::ParticleSystem;
use crate::text::Text;

pub struct GameState {
    pub camera: Camera,
    pub entities: Vec<Entity>,
    pub text: Text,
    pub score_text: Text,
    pub particle_system: ParticleSystem,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            entities: Vec::new(),
            text: Text::new("", 16.0),
            score_text: Text::new("", 16.0),
            particle_system: ParticleSystem::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.particle_system.update(delta_time);

        for (particle, entity) in self
            .particle_system
            .particles
            .iter()
            .zip(self.entities.iter_mut())
        {
            entity
                .transform
                .set_position(particle.position[0], particle.position[1]);
        }
    }
}
