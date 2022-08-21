use std::{cmp::Ordering, rc::Rc};

use rand::prelude::*;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
};

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub aabb: Aabb,
}

impl BvhNode {
    pub fn from_hittable_list(list: HittableList, time0: f64, time1: f64) -> Self {
        Self::from_slice(&list.objects[..], time0, time1)
    }

    pub fn from_slice(src_objects: &[Rc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let mut objects = src_objects.to_owned();
        let comp = [box_compare_x, box_compare_y, box_compare_z]
            .choose(&mut thread_rng())
            .unwrap();
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        if objects.len() == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if objects.len() == 2 {
            if comp(objects[0].clone(), objects[1].clone()) == Ordering::Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_by(|a, b| comp(a.clone(), b.clone()));
            let mid = objects.len() / 2;
            left = Rc::new(Self::from_slice(&objects[0..mid], time0, time1));
            right = Rc::new(Self::from_slice(&objects[mid..], time0, time1));
        }

        let msg = "No bounding box in bvh_node constructor.\n";
        let left_box = left.bounding_box(time0, time1).expect(msg);
        let right_box = right.bounding_box(time0, time1).expect(msg);
        let aabb = left_box + right_box;

        Self { left, right, aabb }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(r, t_min, t_max) {
            return None;
        }

        let mut hit = self.left.hit(r, t_min, t_max);
        let next_hit_t_max = match &hit {
            Some(hit) => hit.t,
            None => t_max,
        };
        hit = self.right.hit(r, t_min, next_hit_t_max).or(hit);
        hit
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.aabb)
    }
}

fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis: usize) -> Ordering {
    let msg = "No bounding box in bvh_node.\n";
    let box_a = a.bounding_box(0.0, 0.0).expect(msg);
    let box_b = b.bounding_box(0.0, 0.0).expect(msg);
    box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
}

fn box_compare_x(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_compare_y(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
fn box_compare_z(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
