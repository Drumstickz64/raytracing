use std::rc::Rc;

use rand::prelude::*;

use crate::{
    aarect::{XYRect, XZRect, YZRect},
    camera::Camera,
    color,
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    math,
    sphere::{MovingSphere, Sphere},
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    ASPECT_RATIO, TIME0, TIME1,
};

pub struct Scene {
    pub world: HittableList,
    pub cam: Camera,
    pub background_color: glam::DVec3,
    pub samples_per_pixel: u32,
    pub image_width: u32,
    pub image_height: u32,
}

impl Scene {
    pub fn new(world: HittableList, cam: Camera) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        Self {
            world,
            cam,
            background_color: color::DEEP_SKY_BLUE,
            samples_per_pixel: 100,
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as u32,
        }
    }

    pub fn with_background_color(mut self, background_color: glam::DVec3) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn with_samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_image_width(mut self, width: u32, aspect_ratio: f64) -> Self {
        self.image_width = width;
        self.image_height = (width as f64 / aspect_ratio) as u32;
        self
    }
}

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

    Scene::new(world, cam)
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

    Scene::new(world, cam)
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

    Scene::new(world, cam)
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

    Scene::new(world, cam)
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

    Scene::new(world, cam)
}

#[allow(dead_code)]
pub fn black_screen() -> Scene {
    let world = HittableList::default();
    let cam = Camera::default();
    Scene::new(world, cam).with_background_color(color::BLACK)
}

#[allow(dead_code)]
pub fn simple_light() -> Scene {
    let lookfrom = glam::dvec3(26.0, 3.0, 6.0);
    let lookat = glam::dvec3(0.0, 2.0, 0.0);
    let vup = glam::dvec3(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 100.0;
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

    let difflight = Rc::new(DiffuseLight::from_color(glam::DVec3::splat(4.0)));
    let rect_light = XYRect {
        mp: difflight.clone(),
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
    };
    let sphere_light = Sphere::new(glam::dvec3(0.0, 7.0, 0.0), 2.0, difflight);
    world.add(Rc::new(rect_light));
    world.add(Rc::new(sphere_light));

    Scene::new(world, cam)
        .with_background_color(color::BLACK)
        .with_samples_per_pixel(400)
}

pub fn cornel_box() -> Scene {
    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 200;
    let background_color = color::BLACK;
    let lookfrom = glam::dvec3(278.0, 278.0, -800.0);
    let lookat = glam::dvec3(278.0, 278.0, 0.0);
    let vup = glam::dvec3(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.0;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        TIME0,
        TIME1,
    );

    let mut world = HittableList::default();

    let red = Rc::new(Lambertian::from_color(glam::dvec3(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::from_color(glam::DVec3::splat(0.73)));
    let green = Rc::new(Lambertian::from_color(glam::dvec3(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from_color(glam::DVec3::splat(15.0)));

    world.add(Rc::new(YZRect {
        mp: green,
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    }));

    world.add(Rc::new(YZRect {
        mp: red,
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    }));

    world.add(Rc::new(XZRect {
        mp: light,
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
    }));

    world.add(Rc::new(XZRect {
        mp: white.clone(),
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    }));

    world.add(Rc::new(XZRect {
        mp: white.clone(),
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    }));

    world.add(Rc::new(XYRect {
        mp: white,
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
    }));

    Scene::new(world, cam)
        .with_background_color(background_color)
        .with_image_width(image_width, aspect_ratio)
        .with_samples_per_pixel(samples_per_pixel)
}
