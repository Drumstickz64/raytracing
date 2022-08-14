use crate::ray::Ray;

pub struct Camera {
    origin: glam::Vec3,
    horizontal: glam::Vec3,
    vertical: glam::Vec3,
    lower_left_corner: glam::Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;

        let origin = glam::vec3(0.0, 0.0, 0.0);
        let horizontal = glam::vec3(viewport_width, 0.0, 0.0);
        let vertical = glam::vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - glam::vec3(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
