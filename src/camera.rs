use rand::{thread_rng, Rng};

use crate::{math, ray::Ray};

#[derive(Debug, Default)]
pub struct Camera {
    origin: glam::DVec3,
    horizontal: glam::DVec3,
    vertical: glam::DVec3,
    lower_left_corner: glam::DVec3,
    #[allow(dead_code)]
    w: glam::DVec3,
    v: glam::DVec3,
    u: glam::DVec3,
    len_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lookfrom: glam::DVec3,
        lookat: glam::DVec3,
        vup: glam::DVec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            w,
            v,
            u,
            len_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = math::random_point_in_unit_disk() * self.len_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            thread_rng().gen_range(self.time0..self.time1),
        )
    }
}
