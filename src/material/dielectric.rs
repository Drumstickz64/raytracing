use crate::{color, hittable::HitRecord, math::VecExtension, ray::Ray};

use super::{Material, MaterialRayInteraction};

use rand::prelude::*;

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // use Shlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> MaterialRayInteraction {
        let attenuation = color::WHITE;
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect =
            cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random();
        let direction = if will_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };
        let ray = Ray::new(rec.point, direction, r_in.time);
        MaterialRayInteraction::Scattered {
            attenuation,
            scattered_ray: ray,
        }
    }
}
