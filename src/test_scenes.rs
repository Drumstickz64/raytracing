use std::rc::Rc;

use rand::prelude::*;

use crate::{
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    math,
    sphere::{MovingSphere, Sphere},
};

#[allow(dead_code)]
pub fn simple_scene() -> HittableList {
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        glam::dvec3(0.0, 0.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(glam::dvec3(0.7, 0.3, 0.3))),
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(-24.0, 0.0, -2.0),
        0.5,
        Rc::new(Lambertian::new(glam::dvec3(0.3, 0.7, 0.3))),
    )));
    world.add(Rc::new(Sphere::new(
        glam::dvec3(6.0, 1.0, 0.0),
        0.5,
        Rc::new(Lambertian::new(glam::dvec3(0.3, 0.3, 0.7))),
    )));

    world
}

#[allow(dead_code)]
pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let mut rng = thread_rng();
    let ground_mat = Rc::new(Lambertian::new(glam::DVec3::splat(0.5)));
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
                    sphere_mat = Rc::new(Lambertian::new(albedo));
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

    let mat2 = Rc::new(Lambertian::new(glam::dvec3(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(glam::dvec3(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Rc::new(Metal::new(glam::dvec3(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(glam::dvec3(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}
