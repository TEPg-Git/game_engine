use crate::sprite::Sprite;
use crate::transform::Transform;

pub struct Entity {
    pub id: u32,
    pub name: String,
    pub transform: Transform,
    pub sprite: Option<Sprite>,
}

impl Entity {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            transform: Transform::new(),
            sprite: None,
        }
    }

    pub fn set_sprite(&mut self, sprite: Sprite) {
        self.sprite = Some(sprite);
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.transform.translate(x, y);
    }
}
