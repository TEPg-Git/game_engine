use std::collections::HashMap;
use wgpu::{Device, Queue};

use crate::texture::Texture;

pub struct AssetManager {
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn load_texture(
        &mut self,
        device: &Device,
        queue: &Queue,
        name: &str,
        path: &str,
    ) -> &Texture {
        if !self.textures.contains_key(name) {
            let texture = Texture::from_file(device, queue, path);
            self.textures.insert(name.to_string(), texture);
        }

        self.textures.get(name).unwrap()
    }
}
