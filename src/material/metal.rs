use crate::{
    hittable::HitRecord,
    math::{self, VecExtension},
    ray::Ray,
};

use super::{Material, MaterialRayInteraction};

pub struct Metal {
    pub albedo: glam::DVec3,
    pub fuzzines: f64,
}

impl Metal {
    pub fn new(albedo: glam::DVec3, fuzzines: f64) -> Self {
        Self {
            albedo,
            fuzzines: fuzzines.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> MaterialRayInteraction {
        let reflection_direction = r_in.direction.normalize().reflect(rec.normal);
        let scattered_ray = Ray::new(
            rec.point,
            reflection_direction + math::random_point_in_unit_sphere() * self.fuzzines,
        );
        let attenuation = self.albedo;
        if scattered_ray.direction.dot(rec.normal) > 0.0 {
            MaterialRayInteraction::Scattered {
                attenuation,
                scattered_ray,
            }
        } else {
            MaterialRayInteraction::Absorbed
        }
    }
}
