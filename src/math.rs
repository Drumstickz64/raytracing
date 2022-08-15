use rand::prelude::*;

pub fn random_range_vec(min: f64, max: f64) -> glam::DVec3 {
    let mut rng = thread_rng();
    glam::dvec3(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_vec_in_unit_sphere() -> glam::DVec3 {
    loop {
        let p = random_range_vec(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vec() -> glam::DVec3 {
    random_vec_in_unit_sphere().normalize()
}

// pub fn random_in_hemisphere(normal: glam::DVec3) -> glam::DVec3 {
//     let in_unit_sphere = random_vec_in_unit_sphere();
//     if in_unit_sphere.dot(normal) > 0.0 {
//         // In the same hemisphere as the normal
//         in_unit_sphere
//     } else {
//         -in_unit_sphere
//     }
// }