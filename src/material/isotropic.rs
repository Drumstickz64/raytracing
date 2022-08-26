use std::rc::Rc;

use crate::{
    hittable::HitRecord,
    math,
    ray::Ray,
    texture::{SolidColor, Texture},
};

use super::{Material, MaterialRayInteraction};

pub struct Isotropic {
    pub albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn from_texture(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from_color(color: glam::DVec3) -> Self {
        Self {
            albedo: Rc::new(SolidColor::from_color(color)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> MaterialRayInteraction {
        let scattered_ray = Ray::new(rec.point, math::random_point_in_unit_sphere(), r_in.time);
        MaterialRayInteraction::Scattered {
            attenuation: self.albedo.value(rec.u, rec.v, rec.point),
            scattered_ray,
        }
    }
}
