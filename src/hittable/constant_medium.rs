use std::rc::Rc;

use rand::random;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::{Isotropic, Material},
    ray::Ray,
    texture::Texture,
};

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub neg_inv_density: f64,
    pub phasing_function: Rc<dyn Material>,
}

impl ConstantMedium {
    #[allow(dead_code)]
    pub fn from_texture(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phasing_function: Rc::new(Isotropic::from_texture(a)),
        }
    }

    pub fn from_color(b: Rc<dyn Hittable>, d: f64, c: glam::DVec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phasing_function: Rc::new(Isotropic::from_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && random::<f64>() < 0.00001;

        let mut rec1 = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY)?;

        if debugging {
            println!("t_min={}, t_max={}", rec1.t, rec2.t);
        }

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        // WARN
        let hit_distance = self.neg_inv_density * random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let rec_t = rec1.t + hit_distance / ray_length;
        let rec_p = r.at(rec_t);

        if debugging {
            println!("hit_distance={}", hit_distance);
            println!("rec.t={}", rec_t);
            println!("rec.p={}", rec_p);
        }

        Some(HitRecord {
            point: rec_p,
            normal: glam::DVec3::Y, // arbitrary
            front_face: true,       // also arbitrary
            mat: Some(self.phasing_function.clone()),
            t: rec_t,
            ..Default::default()
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
