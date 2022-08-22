use std::rc::Rc;

use crate::{
    hittable::HitRecord,
    math::{self, VecExtension},
    ray::Ray,
    texture::{SolidColor, Texture},
};

use super::{Material, MaterialRayInteraction};

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn from_texture(texture: Rc<dyn Texture>) -> Self {
        Self { albedo: texture }
    }

    pub fn from_color(color: glam::DVec3) -> Self {
        Self {
            albedo: Rc::new(SolidColor::from_color(color)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> MaterialRayInteraction {
        let scatter_direction = rec.normal + math::random_unit_vec();
        let scatter_direction = if scatter_direction.is_near_zero() {
            rec.normal
        } else {
            scatter_direction
        };
        let scattered_ray = Ray::new(rec.point, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.point);
        MaterialRayInteraction::Scattered {
            attenuation,
            scattered_ray,
        }
    }
}
