mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod math;
mod noise;
mod ray;
mod sphere;
mod test_scenes;
mod texture;

use std::{fmt::Write, fs};

use indicatif::ProgressBar;
use rand::prelude::*;

use crate::{
    bvh::BvhNode, color::stringify_color, hittable::Hittable, material::MaterialRayInteraction,
    ray::Ray,
};

// Screen
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 50;
const TIME0: f64 = 0.0;
const TIME1: f64 = 1.0;
const OUTPUT_FILE: &str = "out.ppm";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT) as usize * 12 + 20);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    let mut rng = thread_rng();

    // World
    let (world, cam) = test_scenes::earth();
    let world = BvhNode::from_hittable_list(world, TIME0, TIME1);

    write!(&mut buf, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")?;

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
            return match mat.scatter(r, &rec) {
                MaterialRayInteraction::Absorbed => color::BLACK,
                MaterialRayInteraction::Scattered {
                    attenuation,
                    scattered_ray,
                } => attenuation * ray_color(scattered_ray, world, depth - 1),
            };
        }
    }
    // Background
    let unit_direction = r.direction.normalize();
    let delta = (unit_direction.y + 1.0) * 0.5;
    color::WHITE.lerp(color::SKY_BLUE, delta)
}
