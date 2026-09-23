use wgpu::Device;
use wgpu::Queue;

use crate::texture::Texture;

// ============================================================
// SPRITE
// ============================================================

pub struct Sprite {
    // The visual data needed to render a sprite.
    //
    // Texture ownership is kept here for the current prototype.
    // A future asset manager can replace this with a texture handle.
    pub texture: Texture,

    // World-space size of the sprite quad.
    pub size: [f32; 2],
}

impl Sprite {
    pub fn from_file(device: &Device, queue: &Queue, path: &str, size: [f32; 2]) -> Self {
        Self {
            texture: Texture::from_file(device, queue, path),
            size,
        }
    }
}
