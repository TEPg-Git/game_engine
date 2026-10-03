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
