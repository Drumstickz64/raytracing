use super::Texture;

pub struct SolidColor {
    color_value: glam::DVec3,
}

impl SolidColor {
    #[allow(dead_code)]
    pub fn new(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color_value: glam::dvec3(r, g, b),
        }
    }

    pub fn from_color(color_value: glam::DVec3) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: glam::DVec3) -> glam::DVec3 {
        self.color_value
    }
}
