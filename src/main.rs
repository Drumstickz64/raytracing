mod aabb;
mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod noise;
mod ray;
mod test_scenes;
mod texture;

use std::{fmt::Write, fs};

use hittable::BvhNode;
use indicatif::ProgressBar;
use rand::prelude::*;
use test_scenes::Scene;

use crate::{
    color::stringify_color, hittable::Hittable, material::MaterialRayInteraction, ray::Ray,
};

// Screen
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const MAX_DEPTH: i32 = 50;
const TIME0: f64 = 0.0;
const TIME1: f64 = 1.0;
const OUTPUT_FILE: &str = "out.ppm";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // World
    let Scene {
        world,
        cam,
        background_color,
        samples_per_pixel,
        image_width,
        image_height,
    } = test_scenes::final_scene();

    let mut buf = String::with_capacity((image_width * image_height) as usize * 12 + 20);
    let pb = ProgressBar::new(image_height as u64);
    let mut rng = thread_rng();

    let world = BvhNode::from_hittable_list(world, TIME0, TIME1);

    write!(&mut buf, "P3\n{image_width} {image_height}\n255\n")?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = color::BLACK;
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width + 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height + 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, background_color, &world, MAX_DEPTH);
            }
            write!(
                &mut buf,
                "{}",
                stringify_color(pixel_color, samples_per_pixel)
            )?;
        }
        pb.set_position((image_height - j) as u64);
    }
    fs::write(OUTPUT_FILE, buf)?;
    pb.finish_with_message("Done!");
    Ok(())
}

fn ray_color(
    r: Ray,
    background_color: glam::DVec3,
    world: &dyn Hittable,
    depth: i32,
) -> glam::DVec3 {
    if depth <= 0 {
        return color::BLACK;
    }

    if let Some(rec) = world.hit(r, 0.0001, f64::INFINITY) {
        if let Some(mat) = &rec.mat {
            let emitted = mat.emitted(rec.u, rec.v, rec.point);
            return match mat.scatter(r, &rec) {
                MaterialRayInteraction::Absorbed => emitted,
                MaterialRayInteraction::Scattered {
                    attenuation,
                    scattered_ray,
                } => {
                    emitted
                        + attenuation * ray_color(scattered_ray, background_color, world, depth - 1)
                }
            };
        }
    }
    // Background
    background_color
}
