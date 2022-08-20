use crate::{
    hittable::HitRecord,
    math::{self, VecExtension},
    ray::Ray,
};

use super::{Material, MaterialRayInteraction};

pub struct Lambertian {
    pub albedo: glam::DVec3,
}

impl Lambertian {
    pub fn new(albedo: glam::DVec3) -> Self {
        Self { albedo }
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
        let attenuation = self.albedo;
        MaterialRayInteraction::Scattered {
            attenuation,
            scattered_ray,
        }
    }
}
