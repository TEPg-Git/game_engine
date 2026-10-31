use std::collections::HashMap;

use crate::texture::Texture;

// Centralized storage for loaded engine assets.
pub struct AssetManager {
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}
