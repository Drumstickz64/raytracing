pub const WHITE: glam::Vec3 = glam::vec3(1.0, 1.0, 1.0);
pub const BLACK: glam::Vec3 = glam::vec3(0.0, 0.0, 0.0);
pub const BLUE: glam::Vec3 = glam::vec3(0.5, 0.7, 1.0);

pub fn stringify_color(color: glam::Vec3, samples_per_pixel: u32) -> String {
    let scale = 1.0 / samples_per_pixel as f32;
    let color = color * scale;
    format!(
        "{} {} {}",
        (256.0 * color.x.clamp(0.0, 0.999)) as u32,
        (256.0 * color.y.clamp(0.0, 0.999)) as u32,
        (256.0 * color.z.clamp(0.0, 0.999)) as u32
    )
}
