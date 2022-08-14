mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;

use rand::prelude::*;

use color::stringify_color;
use hittable::{HitRecord, Hittable};
use ray::Ray;

use crate::{camera::Camera, hittable_list::HittableList, sphere::Sphere};

// Screen
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let cam = Camera::new();

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(glam::vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(glam::vec3(0.0, -100.5, -1.0), 100.0)));

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\raScanlines remaining: {j} {esc}", esc = 27 as char);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = color::BLACK;
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random::<f32>()) / (IMAGE_WIDTH + 1) as f32;
                let v = (j as f32 + random::<f32>()) / (IMAGE_HEIGHT + 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }
            print!("{}\t\t", stringify_color(pixel_color, SAMPLES_PER_PIXEL));
        }
        println!();
    }
    eprintln!("\naI'm Done!");
}

fn ray_color(r: Ray, world: &dyn Hittable) -> glam::Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f32::INFINITY, &mut rec) {
        return 0.5 * (color::WHITE + rec.normal);
    }
    // Background
    let unit_direction = r.direction.normalize();
    let delta = (unit_direction.y + 1.0) * 0.5;
    color::WHITE.lerp(color::BLUE, delta)
}
