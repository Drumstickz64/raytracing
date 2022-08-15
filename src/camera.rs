use crate::ray::Ray;

pub struct Camera {
    origin: glam::DVec3,
    horizontal: glam::DVec3,
    vertical: glam::DVec3,
    lower_left_corner: glam::DVec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let origin = glam::dvec3(0.0, 0.0, 0.0);
        let horizontal = glam::dvec3(viewport_width, 0.0, 0.0);
        let vertical = glam::dvec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - glam::dvec3(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
