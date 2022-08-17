use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    // pub fn new(object: Rc<dyn Hittable>) -> HittableList {
    //     Self {
    //         objects: vec![object],
    //     }
    // }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
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
}
