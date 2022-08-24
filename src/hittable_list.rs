use std::rc::Rc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let mut world = Self::default();
        world.add(object);
        world
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(obj_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = obj_rec.t;
                rec = Some(obj_rec);
            }
        }
        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut output_box = self.objects[0].bounding_box(time0, time1)?;
        for object in self.objects.iter().skip(1) {
            let aabb = object.bounding_box(time0, time1)?;
            output_box = output_box + aabb;
        }

        Some(output_box)
    }
}
