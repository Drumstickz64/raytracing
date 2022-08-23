use crate::{color, noise::Perlin};

use super::Texture;

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: glam::DVec3) -> glam::DVec3 {
        color::WHITE * 0.5 * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(p, 7)))
    }
}
