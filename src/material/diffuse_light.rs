use std::rc::Rc;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
};

use super::{Material, MaterialRayInteraction};

pub struct DiffuseLight {
    pub emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    // pub fn from_texture(texture: Rc<dyn Texture>) -> Self {
    //     Self { emit: texture }
    // }

    pub fn from_color(color: glam::DVec3) -> Self {
        let emit = Rc::new(SolidColor::from_color(color));
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> MaterialRayInteraction {
        MaterialRayInteraction::Absorbed
    }

    fn emitted(&self, u: f64, v: f64, p: glam::DVec3) -> glam::DVec3 {
        self.emit.value(u, v, p)
    }
}
