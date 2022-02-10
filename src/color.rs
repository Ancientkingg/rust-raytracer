use nalgebra_glm as glm;
use image;

pub fn write_pixel(frag: glm::TVec3<f64>, prev_color: image::Rgba<u8>, samples_per_pixel: u32, frames: i32, timer: u32) -> image::Rgba<u8> {
    let mut frag = frag;
    let scale = 1.0 / samples_per_pixel as f64;
    frag.x = (frag.x * scale).sqrt();
    frag.y = (frag.y * scale).sqrt();
    frag.z = (frag.z * scale).sqrt();
    let new_color = image::Rgba([(frag.x as f64 * 256.) as u8, (frag.y as f64 * 256.) as u8, (frag.z as f64 * 256.) as u8, 255]);
    if timer <= 0 {
        return mix(new_color, prev_color, frames as f64 / (frames as f64 + 1.0));
    }
    new_color
}

fn mix(v: image::Rgba<u8>, w: image::Rgba<u8>, factor: f64) -> image::Rgba<u8> {
    let v = v.0;
    let w = w.0;
    let factor = factor.min(1.0).max(0.0);
    let factor2 = 1.0 - factor;
    let r = (v[0] as f64 * factor2 + w[0] as f64 * factor) as u8;
    let g = (v[1] as f64 * factor2 + w[1] as f64 * factor) as u8;
    let b = (v[2] as f64 * factor2 + w[2] as f64 * factor) as u8;
    let a = (v[3] as f64 * factor2 + w[3] as f64 * factor) as u8;
    image::Rgba([r,g,b,a])
}