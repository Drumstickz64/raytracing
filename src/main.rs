mod color;
mod ray;

use color::stringify_color;
use ray::Ray;

// Screen
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

// Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f32 = 1.0;

const ORIGIN: glam::Vec3 = glam::vec3(0.0, 0.0, 0.0);
const HORIZONTAL: glam::Vec3 = glam::vec3(VIEWPORT_WIDTH, 0.0, 0.0);
const VERTICAL: glam::Vec3 = glam::vec3(0.0, VIEWPORT_HEIGHT, 0.0);

fn main() {
    let lower_left_corner =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - glam::vec3(0.0, 0.0, FOCAL_LENGTH);
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\raScanlines remaining: {j} {esc}", esc = 27 as char);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray::new(
                ORIGIN,
                lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );
            let color = ray_color(r);
            print!("{}\t\t", stringify_color(color));
        }
        println!();
    }
    eprintln!("\naI'm Done!");
}

fn ray_color(r: Ray) -> glam::Vec3 {
    if hit_sphere(glam::vec3(0.0, 0.0, -1.0), 0.5, r) {
        return color::RED;
    }
    let unit_direction = r.direction.normalize();
    let delta = (unit_direction.y + 1.0) * 0.5;
    color::WHITE.lerp(color::BLUE, delta)
}

fn hit_sphere(center: glam::Vec3, radius: f32, r: Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
