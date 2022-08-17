mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod math;
mod ray;
mod sphere;

use std::{fmt::Write, fs, rc::Rc};

use indicatif::ProgressBar;
use material::{lambertian::Lambertian, metal::Metal, MaterialRayInteraction};
use rand::prelude::*;

use crate::{
    camera::Camera, color::stringify_color, hittable::Hittable, hittable_list::HittableList,
    ray::Ray, sphere::Sphere,
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
    let mat_ground = Rc::new(Lambertian::new(color::LIGHT_GRASS_GREEN));
    let mat_center = Rc::new(Lambertian::new(color::LIGHT_CLAY));
    let mat_left = Rc::new(Metal::new(color::LIGHT_GRAY, 0.3));
    let mat_right = Rc::new(Metal::new(color::RUSTY_RED, 1.0));

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, 0.0, -1.0),
        0.5,
        mat_center,
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

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
    if depth <= 0 {
        return color::BLACK;
    }

    if let Some(rec) = world.hit(r, 0.0001, f64::INFINITY) {
        if let Some(mat) = &rec.mat {
            if let MaterialRayInteraction::Scattered {
                attenuation,
                scattered_ray,
            } = mat.scatter(r, &rec)
            {
                return attenuation * ray_color(scattered_ray, world, depth - 1);
            }
        }
        return color::BLACK;
    }
    // Background
    let unit_direction = r.direction.normalize();
    let delta = (unit_direction.y + 1.0) * 0.5;
    color::WHITE.lerp(color::BLUE, delta)
}
