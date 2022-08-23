mod checker;
mod noise;
mod solid_color;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: glam::DVec3) -> glam::DVec3;
}

pub use checker::CheckerTexture;
pub use noise::NoiseTexture;
pub use solid_color::SolidColor;
