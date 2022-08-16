use crate::{hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut glam::DVec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub mod lambertian;
pub mod metal;
