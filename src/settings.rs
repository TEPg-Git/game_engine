#[derive(Clone, Debug)]
pub struct ParticleSettings {
    pub particle_count: usize,
    pub flow_speed: f32,
    pub flow_steering: f32,
    pub field_scale: f32,
    pub field_resolution: usize,
    pub particle_size: f32,
    pub field_update_interval: f32,
}

impl Default for ParticleSettings {
    fn default() -> Self {
        Self {
            particle_count: 100_000,
            flow_speed: 0.3,
            flow_steering: 2.0,
            field_scale: 2.0,
            field_resolution: 64,
            particle_size: 0.0015,
            field_update_interval: 0.5,
        }
    }
}
