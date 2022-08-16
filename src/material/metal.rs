use crate::{
    hittable::HitRecord,
    math::{self, VecExtension},
    ray::Ray,
};

use super::Material;

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
    fn scatter(
        &self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut glam::DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflection_direction = r_in.direction.normalize().reflect(rec.normal);
        *scattered = Ray::new(
            rec.point,
            reflection_direction + math::random_vec_in_unit_sphere() * self.fuzzines,
        );
        *attenuation = self.albedo;
        scattered.direction.dot(rec.normal) > 0.0
    }
}
