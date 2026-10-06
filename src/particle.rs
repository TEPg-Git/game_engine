use crate::flow_field::FlowField;
use rand::Rng;

pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub lifetime: f32,
}

impl Particle {
    pub fn new(position: [f32; 2], velocity: [f32; 2], lifetime: f32) -> Self {
        Self { position, velocity, lifetime }
    }

    pub fn update(&mut self, dt: f32) {
        if self.lifetime <= 0.0 { return; }
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;
        self.lifetime -= dt;
        if self.lifetime <= 0.0 { self.lifetime = 0.0; }
    }

    pub fn update_with_flow(&mut self, dt: f32, flow_field: &FlowField, speed: f32, steering: f32) {
        if self.lifetime <= 0.0 { return; }

        let direction = flow_field.sample(self.position);
        let target_velocity = [direction[0] * speed, direction[1] * speed];
        let blend = (steering.max(0.0) * dt).clamp(0.0, 1.0);

        self.velocity[0] += (target_velocity[0] - self.velocity[0]) * blend;
        self.velocity[1] += (target_velocity[1] - self.velocity[1]) * blend;
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;
        self.lifetime -= dt;

        if self.lifetime <= 0.0 { self.lifetime = 0.0; }
    }

    pub fn is_alive(&self) -> bool { self.lifetime > 0.0 }

    pub fn reset(&mut self) {
        let max_speed = 2.0;
        let max_range = 1.0;
        let mut rng = rand::thread_rng();
<<<<<<< HEAD
        self.position = [
            rng.gen_range(-max_range..=max_range),
            rng.gen_range(-max_range..=max_range),
        ];
        self.velocity = [
            rng.gen_range(-max_speed..=max_speed),
            rng.gen_range(-max_speed..=max_speed),
        ];
        self.lifetime = rng.gen_range(1.0..=10.0);
=======
        self.position = [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)];
        self.velocity = [0.0, 0.0];
        self.lifetime = rng.gen_range(2.0..=10.0);
>>>>>>> 14c041ac6fe0817b17794051bcffdd1ede02a76d
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
        let mut rng = rand::thread_rng();
<<<<<<< HEAD
        let n = 100;
=======
        let n = 500;

>>>>>>> 14c041ac6fe0817b17794051bcffdd1ede02a76d
        Self {
            particles: (0..n)
                .map(|_| Particle::new(
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    [0.0, 0.0],
                    rng.gen_range(2.0..=10.0),
                ))
                .collect(),
            flow_field: FlowField::new(32, 32, 2.5),
            flow_speed: 0.35,
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
                || particle.position[0] < -1.0 || particle.position[0] > 1.0
                || particle.position[1] < -1.0 || particle.position[1] > 1.0
            {
                particle.reset();
            }
        }
    }
}
