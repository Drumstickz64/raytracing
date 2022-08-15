mod camera;
mod color;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;

use std::{fmt::Write, fs};

use indicatif::ProgressBar;
use rand::prelude::*;

use crate::{
    camera::Camera,
    color::stringify_color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
};

// Screen
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 50;
const OUTPUT_FILE: &str = "out.ppm";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cam = Camera::new();
    let mut buf = String::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT) as usize * 12 + 20);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(glam::dvec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(glam::dvec3(0.0, -100.5, -1.0), 100.0)));

    write!(&mut buf, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")?;

    let mut rng = thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = color::BLACK;
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH + 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT + 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            write!(
                &mut buf,
                "{}",
                stringify_color(pixel_color, SAMPLES_PER_PIXEL)
            )?;
        }
        pb.set_position((IMAGE_HEIGHT - j) as u64);
    }
    fs::write(OUTPUT_FILE, buf)?;
    pb.finish_with_message("Done!");
    Ok(())
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> glam::DVec3 {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return color::BLACK;
    }

    if world.hit(r, 0.0001, f64::INFINITY, &mut rec) {
        let target = rec.point + rec.normal + math::random_unit_vec();
        let diffuse_ray = Ray::new(rec.point, target - rec.point);
        return 0.5 * ray_color(diffuse_ray, world, depth - 1);
    }
    // Background
    let unit_direction = r.direction.normalize();
    let delta = (unit_direction.y + 1.0) * 0.5;
    color::WHITE.lerp(color::BLUE, delta)
}
