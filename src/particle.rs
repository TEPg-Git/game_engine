use crate::flow_field::FlowField;
use rand::Rng;

pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub color: [f32; 4],
    pub lifetime: f32,
}

impl Particle {
    pub fn new(position: [f32; 2], velocity: [f32; 2], color: [f32; 4], lifetime: f32) -> Self {
        Self {
            position,
            velocity,
            color,
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

        let speed = (self.velocity[0] * self.velocity[0] + self.velocity[1] * self.velocity[1])
            .sqrt()
            .clamp(0.0, 1.0);

        if speed < 0.3 {
            self.color[0] = 0.16;
            self.color[1] = 0.47;
            self.color[2] = 1.0;
            self.color[3] = 0.2;
        } else if speed < 0.6 {
            self.color[0] = 0.0;
            self.color[1] = 0.85;
            self.color[2] = 1.0;
            self.color[3] = 0.6;
        } else {
            self.color[0] = 1.0;
            self.color[1] = 0.85;
            self.color[2] = 0.2;
            self.color[3] = 1.0;
        }
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
        let max_speed = 0.0;
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
    /// Total simulation time, in seconds, independent of frame delta.
    elapsed_time: f32,
    /// Seconds between flow field rebuilds. Reduces the cost of rebuilding a large grid every frame.
    field_update_interval: f32,
    field_update_accumulator: f32,
    field_scale: f32,
}

impl ParticleSystem {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        const PARTICLE_COUNT: usize = 100000;
        let field_scale = 3.0;

        Self {
            particles: (0..PARTICLE_COUNT)
                .map(|_| {
                    Particle::new(
                        [rng.random_range(-0.5..=0.5), rng.random_range(-0.5..=0.5)],
                        [0.0, 0.0],
                        [1.0, 1.0, 1.0, 1.0],
                        rng.random_range(2.0..=10.0),
                    )
                })
                .collect(),
            flow_field: FlowField::new(1024, 1024, field_scale, 9678498),
            flow_speed: 1.0,
            flow_steering: 2.0,
            elapsed_time: 0.0,
            // 20 updates/sec by default; tune this down if field generation costs too much.
            field_update_interval: 0.05,
            field_update_accumulator: 0.0,
            field_scale,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let dt = dt.max(0.0);
        self.elapsed_time += dt;
        self.field_update_accumulator += dt;

        if self.field_update_accumulator >= self.field_update_interval {
            // Preserve leftover time so the update cadence does not drift.
            self.field_update_accumulator %= self.field_update_interval;
            self.flow_field
                .generate_at_time(self.field_scale, self.elapsed_time);
        }

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
