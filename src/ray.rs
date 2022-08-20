#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: glam::DVec3,
    pub direction: glam::DVec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: glam::DVec3, direction: glam::DVec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> glam::DVec3 {
        self.origin + self.direction * t
    }
}
