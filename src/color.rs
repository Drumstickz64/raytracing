pub const WHITE: glam::DVec3 = glam::dvec3(1.0, 1.0, 1.0);
pub const BLACK: glam::DVec3 = glam::dvec3(0.0, 0.0, 0.0);
// pub const SKY_BLUE: glam::DVec3 = glam::dvec3(0.5, 0.7, 1.0);
pub const DEEP_SKY_BLUE: glam::DVec3 = glam::dvec3(0.7, 0.8, 1.0);

pub fn stringify_color(color: glam::DVec3, samples_per_pixel: u32) -> String {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();
    format!(
        "{} {} {}\n",
        (256.0 * r.clamp(0.0, 0.999)) as u32,
        (256.0 * g.clamp(0.0, 0.999)) as u32,
        (256.0 * b.clamp(0.0, 0.999)) as u32
    )
}
