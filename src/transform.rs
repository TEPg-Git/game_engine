// ============================================================
// TRANSFORM
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
    revision: u64,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
            revision: 0,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.position[0] += x;
        self.position[1] += y;
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn rotate(&mut self, radians: f32) {
        self.rotation += radians;
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = [x, y];
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }
}
