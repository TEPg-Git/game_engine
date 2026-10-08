use crate::flow_field::FlowField;
use rand::Rng;

pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub lifetime: f32,
}

impl Particle {
    pub fn new(position: [f32; 2], velocity: [f32; 2], lifetime: f32) -> Self {
        Self {
            position,
            velocity,
            lifetime,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.lifetime <= 0.0 {
            return;
        }
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;
        self.lifetime = (self.lifetime - dt).max(0.0);
    }

    pub fn update_with_flow(&mut self, dt: f32, flow_field: &FlowField, speed: f32, steering: f32) {
        if self.lifetime <= 0.0 {
            return;
        }

        let direction = flow_field.sample(self.position);
        let target_velocity = [direction[0] * speed, direction[1] * speed];
        let blend = (steering.max(0.0) * dt).clamp(0.0, 1.0);

        self.velocity[0] += (target_velocity[0] - self.velocity[0]) * blend;
        self.velocity[1] += (target_velocity[1] - self.velocity[1]) * blend;

        self.update(dt);
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn reset(&mut self) {
        let max_speed = 1.0;
        let max_range = 1.0;
        let mut rng = rand::rng();
        self.position = [
            rng.random_range(-max_range..=max_range),
            rng.random_range(-max_range..=max_range),
        ];
        self.velocity = [
            rng.random_range(-max_speed..=max_speed),
            rng.random_range(-max_speed..=max_speed),
        ];
        self.lifetime = rng.random_range(2.0..=10.0);
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub flow_field: FlowField,
    pub flow_speed: f32,
    pub flow_steering: f32,
}

impl ParticleSystem {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        const PARTICLE_COUNT: usize = 1000;
        Self {
            particles: (0..PARTICLE_COUNT)
                .map(|_| {
                    Particle::new(
                        [rng.random_range(-0.5..=0.5), rng.random_range(-0.5..=0.5)],
                        [0.0, 0.0],
                        rng.random_range(2.0..=10.0),
                    )
                })
                .collect(),
            flow_field: FlowField::new(48, 48, 3.0, 927538),
            flow_speed: 2.0,
            flow_steering: 5.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let flow_field = &self.flow_field;
        let speed = self.flow_speed;
        let steering = self.flow_steering;

        for particle in &mut self.particles {
            particle.update_with_flow(dt, flow_field, speed, steering);

            if !particle.is_alive()
                || particle.position[0] < -1.0
                || particle.position[0] > 1.0
                || particle.position[1] < -1.0
                || particle.position[1] > 1.0
            {
                particle.reset();
            }
        }
    }
}
