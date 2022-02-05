use image;
use piston_window;
use nalgebra_glm as glm;

// use piston_window::EventLoop;

use std::f64;
use rand;
// use std::time::Instant;

mod fps_counter;
mod ray;
mod color;
mod objects;
mod sphere;
mod camera;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const SAMPLES_PER_PIXEL: u32 = 10;

fn main() {
    let mut frame_buffer: image::RgbaImage =
        image::RgbaImage::from_pixel(WIDTH, HEIGHT, image::Rgba([0, 255, 0, 255]));

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Raytracer", [WIDTH, HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|_e| panic!("Could not create window!"));


    let mut tex_context = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut tex = piston_window::Texture::from_image(
        &mut tex_context,
        &frame_buffer,
        &piston_window::TextureSettings::new(),
    )
    .unwrap();

    // window.set_lazy(true);
    // let _counter = Instant::now();
    // let mut fps_counter = fps_counter::FpsCounter::new();

    //* WORLD
    let mut world = objects::HittableList::default();
    world.push(sphere::Sphere::new(glm::vec3(0.0, 0.0, -1.0), 0.5));
    world.push(sphere::Sphere::new(glm::vec3(0.0, -100.5, -1.0), 100.0));

    //* CAMERA
    let camera: camera::Camera = camera::Camera::default();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {

            // let fps = fps_counter.tick();
            // println!("FPS: {}", fps);

            piston_window::clear([1.0; 4], g);
            for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
                let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);
                for _i in 0..SAMPLES_PER_PIXEL {
                    let screen_coords = glm::vec2((x as f64 + rand::random::<f64>()) / WIDTH as f64, 1. - ((y as f64 + rand::random::<f64>()) / HEIGHT as f64));
                    let ray: ray::Ray = camera.get_ray(screen_coords);
                    pixel_color += ray::ray_color(&ray, &world);
                }
                *pixel = color::write_pixel(pixel_color, SAMPLES_PER_PIXEL);

            }
            tex.update(&mut tex_context, &frame_buffer).unwrap();
            piston_window::image(&tex, c.transform, g);
            tex_context.encoder.flush(device);
        });
    }
}