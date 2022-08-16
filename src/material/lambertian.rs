use crate::{
    hittable::HitRecord,
    math::{self, VecExtension},
    ray::Ray,
};

use super::Material;

pub struct Lambertian {
    pub albedo: glam::DVec3,
}

impl Lambertian {
    pub fn new(albedo: glam::DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut glam::DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + math::random_unit_vec();
        let scatter_direction = if scatter_direction.is_near_zero() {
            rec.normal
        } else {
            scatter_direction
        };
        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
