mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod math;
mod ray;
mod sphere;

use std::{fmt::Write, fs, sync::Arc};

use indicatif::ProgressBar;
use rand::prelude::*;
use rayon::prelude::*;

use crate::{
    camera::Camera,
    color::stringify_color,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{
        dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material,
        MaterialRayInteraction,
    },
    ray::Ray,
    sphere::Sphere,
};

// Screen
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: i32 = 50;
const OUTPUT_FILE: &str = "out.ppm";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT) as usize * 12 + 20);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);

    // World
    let world = random_scene();

    // Camera
    let lookfrom = glam::dvec3(13.0, 2.0, 3.0);
    let lookat = glam::dvec3(0.0, 0.0, 0.0);
    let vup = glam::dvec3(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    write!(&mut buf, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")?;

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_| {
                    let mut rng = thread_rng();
                    let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH + 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT + 1) as f64;
                    let r = cam.get_ray(u, v);
                    ray_color(r, &world, MAX_DEPTH)
                })
                .reduce(
                    || color::BLACK,
                    |pixel_color, sample_color| pixel_color + sample_color,
                );
            write!(
                &mut buf,
                "{}",
                stringify_color(pixel_color, SAMPLES_PER_PIXEL)
            )?;
        }
        pb.set_position((IMAGE_HEIGHT - j) as u64);
    }
    fs::write(OUTPUT_FILE, &buf)?;
    pb.finish_with_message("Done!");
    Ok(())
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> glam::DVec3 {
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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let mut rng = thread_rng();
    let ground_mat = Arc::new(Lambertian::new(glam::DVec3::splat(0.5)));
    world.add(Arc::new(Sphere::new(
        glam::dvec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = glam::dvec3(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            if center.distance(glam::dvec3(4.0, 0.2, 0.0)) > 0.9 {
                let sphere_mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random::<glam::DVec3>() * random::<glam::DVec3>();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = math::random_range_vec(0.5, 1.0);
                    let fuzzines = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzzines))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)))
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(glam::dvec3(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Arc::new(Lambertian::new(glam::dvec3(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        glam::dvec3(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat3 = Arc::new(Metal::new(glam::dvec3(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(glam::dvec3(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}
