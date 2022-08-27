use std::rc::Rc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Translate {
    pub ptr: Rc<dyn Hittable>,
    pub offset: glam::DVec3,
}

impl Translate {
    pub fn new(ptr: Rc<dyn Hittable>, offset: glam::DVec3) -> Self {
        Self { ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        let mut rec = self.ptr.hit(moved_r, t_min, t_max)?;
        rec.point += self.offset;
        rec.set_face_normal(moved_r, rec.normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let ptr_box = self.ptr.bounding_box(time0, time1)?;
        Some(Aabb::new(
            ptr_box.min + self.offset,
            ptr_box.max + self.offset,
        ))
    }
}

pub struct RotateY {
    pub ptr: Rc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(ptr: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = ptr.bounding_box(0.0, 1.0);

        let mut min = glam::DVec3::splat(f64::INFINITY);
        let mut max = glam::DVec3::splat(f64::NEG_INFINITY);

        if let Some(bbox) = bbox {
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max.x + (1 - i) as f64 * bbox.min.x;
                        let y = j as f64 * bbox.max.y + (1 - j) as f64 * bbox.min.y;
                        let z = k as f64 * bbox.max.z + (1 - k) as f64 * bbox.min.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = glam::dvec3(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
        }

        Self {
            ptr,
            sin_theta,
            cos_theta,
            bbox: Some(Aabb::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin[0] = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin[2] = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];

        direction[0] = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction[2] = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];

        let rotated_r = Ray::new(origin, direction, r.time);

        let mut rec = self.ptr.hit(rotated_r, t_min, t_max)?;

        let mut p = rec.point;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.point[0] + self.sin_theta * rec.point[2];
        p[2] = -self.sin_theta * rec.point[0] + self.cos_theta * rec.point[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.point = p;
        rec.set_face_normal(rotated_r, normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
