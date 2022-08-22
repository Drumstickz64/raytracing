use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: glam::DVec3) -> glam::DVec3;
}

pub struct SolidColor {
    color_value: glam::DVec3,
}

impl SolidColor {
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

pub struct CheckerTexture {
    odd_texture: Rc<dyn Texture>,
    even_texture: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd_texture: Rc<dyn Texture>, even_texture: Rc<dyn Texture>) -> Self {
        Self {
            odd_texture,
            even_texture,
        }
    }

    pub fn from_colors(a: glam::DVec3, b: glam::DVec3) -> Self {
        Self {
            odd_texture: Rc::new(SolidColor::from_color(a)),
            even_texture: Rc::new(SolidColor::from_color(b)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: glam::DVec3) -> glam::DVec3 {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            self.odd_texture.value(u, v, p)
        } else {
            self.even_texture.value(u, v, p)
        }
    }
}
