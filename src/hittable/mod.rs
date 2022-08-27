mod aarect;
mod bvh;
mod constant_medium;
mod geometric_box;
mod hittable_list;
mod instance;
pub mod sphere;

use std::rc::Rc;

use crate::{aabb::Aabb, material::Material, ray::Ray};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub point: glam::DVec3,
    pub normal: glam::DVec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn with_face_normal(mut self, r: Ray, outward_normal: glam::DVec3) -> Self {
        self.set_face_normal(r, outward_normal);
        self
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: glam::DVec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub use aarect::{XYRect, XZRect, YZRect};
pub use bvh::BvhNode;
pub use constant_medium::ConstantMedium;
pub use geometric_box::GeometricBox;
pub use hittable_list::HittableList;
pub use instance::{RotateY, Translate};
