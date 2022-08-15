use rand::prelude::*;

pub fn random_range_vec(min: f32, max: f32) -> glam::Vec3 {
    let mut rng = thread_rng();
    glam::Vec3::splat(rng.gen_range(min..max))
}

pub fn random_vec_in_unit_sphere() -> glam::Vec3 {
    loop {
        let p = random_range_vec(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
