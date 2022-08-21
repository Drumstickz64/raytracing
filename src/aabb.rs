use std::ops::Add;

use crate::ray::Ray;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Aabb {
    pub min: glam::DVec3,
    pub max: glam::DVec3,
}

impl Aabb {
    pub fn new(min: glam::DVec3, max: glam::DVec3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_dir = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_dir;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_dir;
            if inv_dir < 0.0 {
                (t0, t1) = (t1, t0);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

impl Add for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Self) -> Self::Output {
        let small = glam::dvec3(
            f64::min(self.min.x, rhs.min.x),
            f64::min(self.min.y, rhs.min.y),
            f64::min(self.min.z, rhs.min.z),
        );

        let big = glam::dvec3(
            f64::max(self.max.x, rhs.max.x),
            f64::max(self.max.y, rhs.max.y),
            f64::max(self.max.z, rhs.max.z),
        );

        Aabb::new(small, big)
    }
}
