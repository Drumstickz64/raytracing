use crate::{hittable::HitRecord, ray::Ray};

pub enum MaterialRayInteraction {
    Absorbed,
    Scattered {
        attenuation: glam::DVec3,
        scattered_ray: Ray,
    },
}

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> MaterialRayInteraction;
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;
