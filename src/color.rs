pub const WHITE: glam::Vec3 = glam::vec3(1.0, 1.0, 1.0);
pub const RED: glam::Vec3 = glam::vec3(1.0, 0.0, 0.0);
pub const BLUE: glam::Vec3 = glam::vec3(0.5, 0.7, 1.0);

pub fn stringify_color(color: glam::Vec3) -> String {
    format!(
        "{} {} {}",
        (color.x * 255.999) as u32,
        (color.y * 255.999) as u32,
        (color.z * 255.999) as u32
    )
}
