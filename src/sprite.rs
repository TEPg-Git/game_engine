use wgpu::Device;
use wgpu::Queue;

use crate::texture::Texture;

// ============================================================
// SPRITE
// ============================================================

pub struct Sprite {
    pub texture: Texture,

    pub size: [f32; 2],
}

// ============================================================
// IMPLEMENTATION
// ============================================================

impl Sprite {
    // ========================================================
    // CREATE SPRITE FROM FILE
    // ========================================================

    pub fn from_file(device: &Device, queue: &Queue, path: &str, size: [f32; 2]) -> Self {
        let texture = Texture::from_file(device, queue, path);

        Self { texture, size }
    }
}
