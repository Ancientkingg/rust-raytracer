use image;
use piston_window::{self, Transformed};
use nalgebra_glm as glm;
use find_folder;

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
mod util;
mod materials;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const SAMPLES_PER_PIXEL: u32 = 3;
const RAY_DEPTH: u8 = 50;

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
    let mut fps_counter = fps_counter::FpsCounter::new();

    //* WORLD
    let mut world = objects::HittableList::default();
    
    let material_ground = materials::Lambertian::new(glm::vec3(0.8, 0.8, 0.0));
    let material_centre = materials::Lambertian::new(glm::vec3(0.1, 0.2, 0.5));
    let material_left =       materials::Dielectric::new(1.5);
    let material_right =      materials::Metal::new(glm::vec3(0.8,0.6,0.2), 0.0);

    world.push(sphere::Sphere::new(glm::vec3(0.0, -100.5, -1.0), 100.0, material_ground));
    world.push(sphere::Sphere::new(glm::vec3(0.0, 0.0, -1.0), 0.5, material_centre));
    world.push(sphere::Sphere::new(glm::vec3(-1.0, 0.0, -1.0), 0.5, material_left));
    world.push(sphere::Sphere::new(glm::vec3(1.0, 0.0, -1.0), 0.5, material_right));
    //* CAMERA
    let camera: camera::Camera = camera::Camera::default();


    //* TEXT
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
        
            piston_window::clear([1.0; 4], g);
            for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
                let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);
                for _i in 0..SAMPLES_PER_PIXEL {
                    let screen_coords = glm::vec2((x as f64 + rand::random::<f64>()) / WIDTH as f64, 1. - ((y as f64 + rand::random::<f64>()) / HEIGHT as f64));
                    let ray: ray::Ray = camera.get_ray(screen_coords);
                    pixel_color += ray::ray_color(&ray, &world, RAY_DEPTH);
                }
                *pixel = color::write_pixel(pixel_color, SAMPLES_PER_PIXEL);

            }
            tex.update(&mut tex_context, &frame_buffer).unwrap();
            piston_window::image(&tex, c.transform, g);
            tex_context.encoder.flush(device);


            
            //* FPS Counter
            let fps = (fps_counter.tick() * 10.0).round() / 10.0;
            let fps = fps.to_string();
            let transform = c.transform.trans(10.0, 30.0);
            piston_window::text::Text::new(32).draw(
                &fps,
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();
            glyphs.factory.encoder.flush(device);
        });
    }
}