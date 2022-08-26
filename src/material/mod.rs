mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

use crate::{color, hittable::HitRecord, ray::Ray};

pub enum MaterialRayInteraction {
    Absorbed,
    Scattered {
        attenuation: glam::DVec3,
        scattered_ray: Ray,
    },
}

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> MaterialRayInteraction;
    fn emitted(&self, _u: f64, _v: f64, _p: glam::DVec3) -> glam::DVec3 {
        color::BLACK
    }
}

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;
