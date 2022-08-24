use std::rc::Rc;

use rand::prelude::*;

use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    math,
    sphere::{MovingSphere, Sphere},
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    ASPECT_RATIO, TIME0, TIME1,
};

type Scene = (HittableList, Camera);

#[allow(dead_code)]
pub fn simple_scene() -> Scene {
    let lookfrom = glam::DVec3::ZERO;
    let lookat = glam::dvec3(0.0, 0.0, 1.0);
    let vup = glam::dvec3(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.1;
    let focus_dist = 1.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
        TIME0,
        TIME1,
    );
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        glam::dvec3(-0.5, 0.0, 1.0),
        0.1,
        Rc::new(Lambertian::from_color(glam::dvec3(0.7, 0.3, 0.3))),
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.5, 0.0, 1.0),
        0.1,
        Rc::new(Lambertian::from_color(glam::dvec3(0.3, 0.7, 0.3))),
    )));

    (world, cam)
}

#[allow(dead_code)]
pub fn random_scene() -> Scene {
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
        TIME0,
        TIME1,
    );
    let mut world = HittableList::default();
    let mut rng = thread_rng();

    let checker_texture =
        CheckerTexture::from_colors(glam::dvec3(0.2, 0.3, 0.1), glam::dvec3(0.9, 0.9, 0.9));
    let ground_mat = Rc::new(Lambertian::from_texture(Rc::new(checker_texture)));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = glam::dvec3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if center.distance(glam::dvec3(4.0, 0.2, 0.0)) > 0.9 {
                let sphere_mat: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let center2 = center + glam::dvec3(0.0, rng.gen_range(0.0..0.5), 0.0);
                    let albedo = rng.gen::<glam::DVec3>() * rng.gen::<glam::DVec3>();
                    sphere_mat = Rc::new(Lambertian::from_color(albedo));
                    world.add(Rc::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, sphere_mat,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = math::random_range_vec(0.5, 1.0);
                    let fuzzines = rng.gen_range(0.0..0.5);
                    sphere_mat = Rc::new(Metal::new(albedo, fuzzines));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    // glass
                    sphere_mat = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(glam::dvec3(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Rc::new(Lambertian::from_color(glam::dvec3(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(glam::dvec3(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Rc::new(Metal::new(glam::dvec3(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(glam::dvec3(4.0, 1.0, 0.0), 1.0, mat3)));

    (world, cam)
}

#[allow(dead_code)]
pub fn two_spheres() -> Scene {
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
        TIME0,
        TIME1,
    );

    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_colors(
        glam::dvec3(0.2, 0.3, 0.1),
        glam::dvec3(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from_texture(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::from_texture(checker)),
    )));

    (world, cam)
}

#[allow(dead_code)]
pub fn two_perlin_spheres() -> Scene {
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
        TIME0,
        TIME1,
    );

    let mut world = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new().with_scale(4.0));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::from_texture(pertext)),
    )));

    (world, cam)
}

#[allow(dead_code)]
pub fn earth() -> Scene {
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
        TIME0,
        TIME1,
    );

    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_mat = Rc::new(Lambertian::from_texture(earth_texture));
    let globe = Rc::new(Sphere::new(glam::DVec3::ZERO, 2.0, earth_mat));
    let world = HittableList::new(globe);

    (world, cam)
}
