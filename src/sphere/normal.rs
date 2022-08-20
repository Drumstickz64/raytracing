use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: glam::DVec3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: glam::DVec3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = r.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let mut rec = HitRecord {
            point,
            normal: glam::DVec3::default(),
            mat: Some(self.mat.clone()),
            t: root,
            front_face: false,
        };

        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
