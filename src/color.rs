use nalgebra_glm as glm;
use image;

pub fn write_pixel(frag: glm::TVec3<f64>, samples_per_pixel: u32) -> image::Rgba<u8> {
    let mut frag = frag;
    let scale = 1.0 / samples_per_pixel as f64;
    frag.x = (frag.x * scale).sqrt();
    frag.y = (frag.y * scale).sqrt();
    frag.z = (frag.z * scale).sqrt();
    image::Rgba([(frag.x as f64 * 256.) as u8, (frag.y as f64 * 256.) as u8, (frag.z as f64 * 256.) as u8, 255])
}