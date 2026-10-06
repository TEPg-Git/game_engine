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

    pub fn reset(&mut self) {
        let max_speed = 2.0;
        let max_range = 1.0;
        let mut rng = rand::thread_rng();
        self.position = [
            rng.gen_range(-max_range..=max_range),
            rng.gen_range(-max_range..=max_range),
        ];
        self.velocity = [
            rng.gen_range(-max_speed..=max_speed),
            rng.gen_range(-max_speed..=max_speed),
        ];
        self.lifetime = rng.gen_range(1.0..=10.0);
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let n = 100;
        Self {
            particles: (0..n)
                .map(|_| {
                    Particle::new(
                        [rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5)],
                        [rng.gen_range(-1.5..=1.5), rng.gen_range(-1.5..=1.5)],
                        rng.gen_range(2.0..=10.0),
                    )
                })
                .collect(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
            if !particle.is_alive() {
                particle.reset();
            }
            if particle.position[0] < -1.0 || particle.position[0] > 1.0 {
                particle.reset();
            }
            if particle.position[1] < -1.0 || particle.position[1] > 1.0 {
                particle.reset();
            }
        }
    }
}
