use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::TAU;

/// A 2D vector field sampled over a regular grid.
/// World coordinates are expected to be approximately [-1, 1].
#[derive(Clone, Debug)]
pub struct FlowField {
    seed: u64,
    width: usize,
    height: usize,
    cell_width: f32,
    cell_height: f32,
    vectors: Vec<[f32; 2]>,
}

impl FlowField {
    pub fn new(width: usize, height: usize, scale: f32, seed: u64) -> Self {
        assert!(
            width > 0 && height > 0,
            "Flow field dimensions must be non-zero"
        );
        let cell_width = 2.0 / width as f32;
        let cell_height = 2.0 / height as f32;

        let mut field = Self {
            seed,
            width,
            height,
            cell_width,
            cell_height,
            vectors: vec![[0.0, 1.0]; width * height],
        };

        field.generate(scale.max(0.001));
        field
    }

    /// Generates a smooth deterministic vector field without an extra noise dependency.
    pub fn generate(&mut self, scale: f32) {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let offset_x = rng.random_range(0.0..TAU);
        let offset_y = rng.random_range(0.0..TAU);
        for y in 0..self.height {
            for x in 0..self.width {
                let world_x = -1.0 + (x as f32 + 0.5) * self.cell_width;
                let world_y = -1.0 + (y as f32 + 0.5) * self.cell_height;

                let sx = world_x * scale;
                let sy = world_y * scale;

                let angle = ((sx + offset_x).sin() * (sy + offset_y).cos()
                    + ((sx + offset_x) * 0.5).cos() * ((sy + offset_y) * 0.75).sin())
                    * TAU;
                let index = self.index(x, y);
                self.vectors[index] = [angle.cos(), angle.sin()];
            }
        }
    }

    /// Bilinearly samples the field at a world-space position.
    pub fn sample(&self, position: [f32; 2]) -> [f32; 2] {
        let gx = ((position[0] + 1.0) / self.cell_width - 0.5)
            .clamp(0.0, self.width.saturating_sub(1) as f32);
        let gy = ((position[1] + 1.0) / self.cell_height - 0.5)
            .clamp(0.0, self.height.saturating_sub(1) as f32);

        let x0 = gx.floor() as usize;
        let y0 = gy.floor() as usize;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let tx = gx - x0 as f32;
        let ty = gy - y0 as f32;

        let top = lerp2(self.vector(x0, y0), self.vector(x1, y0), tx);
        let bottom = lerp2(self.vector(x0, y1), self.vector(x1, y1), tx);

        normalize(lerp2(top, bottom, ty))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn vector(&self, x: usize, y: usize) -> [f32; 2] {
        self.vectors[self.index(x, y)]
    }
}

fn lerp2(a: [f32; 2], b: [f32; 2], t: f32) -> [f32; 2] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
}

fn normalize(v: [f32; 2]) -> [f32; 2] {
    let length_squared = v[0] * v[0] + v[1] * v[1];

    if length_squared <= f32::EPSILON {
        return [0.0, 1.0];
    }

    let inverse_length = length_squared.sqrt().recip();
    [v[0] * inverse_length, v[1] * inverse_length]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_is_normalized() {
        let field = FlowField::new(32, 32, 2.5, 12345);
        let vector = field.sample([0.15, -0.25]);
        let length = (vector[0] * vector[0] + vector[1] * vector[1]).sqrt();

        assert!((length - 1.0).abs() < 0.0001);
    }

    #[test]
    fn samples_are_finite_outside_bounds() {
        let field = FlowField::new(8, 8, 2.0, 12345);
        let vector = field.sample([100.0, -100.0]);

        assert!(vector[0].is_finite());
        assert!(vector[1].is_finite());
    }
    #[test]
    fn same_seed_produces_same_field() {
        let a = FlowField::new(32, 32, 2.5, 12345);
        let b = FlowField::new(32, 32, 2.5, 12345);

        assert_eq!(a.vectors, b.vectors);
    }
    #[test]
    fn different_seeds_produce_different_fields() {
        let a = FlowField::new(32, 32, 2.5, 12345);
        let b = FlowField::new(32, 32, 2.5, 54321);

        assert_ne!(a.vectors, b.vectors);
    }
}
