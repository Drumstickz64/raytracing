use std::rc::Rc;

use crate::texture::{SolidColor, Texture};

pub struct CheckerTexture {
    odd_texture: Rc<dyn Texture>,
    even_texture: Rc<dyn Texture>,
}

impl CheckerTexture {
    #[allow(dead_code)]
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
