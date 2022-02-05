use nalgebra_glm as glm;
use image;

pub fn write_pixel(frag: glm::TVec3<f64>, samples_per_pixel: u32) -> image::Rgba<u8> {
    image::Rgba([(frag.x / samples_per_pixel as f64 * 255.) as u8, (frag.y / samples_per_pixel as f64 * 255.) as u8, (frag.z / samples_per_pixel as f64 * 255.) as u8, 255])
}