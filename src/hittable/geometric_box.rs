use std::rc::Rc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    hittable::{HittableList, XYRect, XZRect, YZRect},
    material::Material,
    ray::Ray,
};

pub struct GeometricBox {
    pub box_min: glam::DVec3,
    pub box_max: glam::DVec3,
    pub sides: HittableList,
}

impl GeometricBox {
    pub fn new(p0: glam::DVec3, p1: glam::DVec3, mat: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::default();

        sides.add(Rc::new(XYRect {
            mp: mat.clone(),
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p1.z,
        }));

        sides.add(Rc::new(XYRect {
            mp: mat.clone(),
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p0.z,
        }));

        sides.add(Rc::new(XZRect {
            mp: mat.clone(),
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p1.y,
        }));

        sides.add(Rc::new(XZRect {
            mp: mat.clone(),
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p0.y,
        }));

        sides.add(Rc::new(YZRect {
            mp: mat.clone(),
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p0.x,
        }));

        sides.add(Rc::new(YZRect {
            mp: mat.clone(),
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p1.x,
        }));

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for GeometricBox {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
