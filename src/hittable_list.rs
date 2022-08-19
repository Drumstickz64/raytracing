use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
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
