use std::{
    f64::consts::{PI, TAU},
    rc::Rc,
};

use crate::{
    aabb::Aabb,
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

    fn get_sphere_uv(p: glam::DVec3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let theta = p.y.acos();
        let phi = f64::atan2(p.z, -p.x);

        (phi / TAU, theta / PI)
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
        let (u, v) = Self::get_sphere_uv(point);
        let mut rec = HitRecord {
            point,
            normal: glam::DVec3::default(),
            mat: Some(self.mat.clone()),
            t: root,
            u,
            v,
            front_face: false,
        };

        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let center_to_edge = glam::DVec3::splat(self.radius);
        Some(Aabb::new(
            self.center - center_to_edge,
            self.center + center_to_edge,
        ))
    }
}
