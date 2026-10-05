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
        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            self.lifetime = 0.0;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            particles: vec![
                Particle::new(
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    rng.gen_range(1.0..=10.0),
                ),
                Particle::new(
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    rng.gen_range(1.0..=10.0),
                ),
                Particle::new(
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    rng.gen_range(1.0..=10.0),
                ),
                Particle::new(
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    rng.gen_range(1.0..=10.0),
                ),
                Particle::new(
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                    rng.gen_range(1.0..=10.0),
                ),
            ],
        }
    }

    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
    }
}
