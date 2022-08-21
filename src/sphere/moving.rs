use std::rc::Rc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
};

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: glam::DVec3,
    pub center1: glam::DVec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: glam::DVec3,
        center1: glam::DVec3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat,
        }
    }

    pub fn center(&self, time: f64) -> glam::DVec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(r.time);
        let oc = r.origin - center;
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
        let outward_normal = (point - center) / self.radius;
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let center_to_edge = glam::DVec3::splat(self.radius);
        let center0 = self.center(time0);
        let box0 = Aabb::new(center0 - center_to_edge, center0 + center_to_edge);
        let center1 = self.center(time1);
        let box1 = Aabb::new(center1 - center_to_edge, center1 + center_to_edge);
        Some(box0 + box1)
    }
}
