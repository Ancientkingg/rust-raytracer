use find_folder;
use nalgebra_glm as glm;
use piston_window::Input::Button;
use piston_window::{self, Transformed};
use piston_window::{AdvancedWindow, Event::*};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use rayon;

use rand::{self, Rng};
use std::{f64, thread};

mod camera;
mod color;
mod fps_counter;
mod materials;
mod objects;
mod ray;
mod sphere;
mod util;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const SAMPLES_PER_PIXEL: u32 = 1;
const RAY_DEPTH: u8 = 50;
const FRAME_TIME: u128 = 50;

fn main() {
    let mut frame_buffer: image::RgbaImage =
        image::RgbaImage::from_pixel(WIDTH, HEIGHT, image::Rgba([0, 0, 0, 255]));

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Raytracer", [WIDTH, HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|_e| panic!("Could not create window!"));

    window.set_capture_cursor(true);
    let mut tex_context = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
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

    //* MULTI THREADING
    // let n_workers = 4;
    // let n_jobs = 8;
    // let pool = ThreadPool::new(n_workers);
    // let (tx, rx) = channel();

    //* WORLD
    let mut world = objects::HittableList::default();

    let material_ground = materials::Lambertian::new(glm::vec3(0.8, 0.8, 0.0));
    let material_centre = materials::Lambertian::new(glm::vec3(0.1, 0.2, 0.5));
    let material_left = materials::Dielectric::new(1.5);
    let material_right = materials::Metal::new(glm::vec3(0.8, 0.6, 0.2), 0.0);

    world.push(sphere::Sphere::new(
        glm::vec3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.push(sphere::Sphere::new(
        glm::vec3(0.0, 0.0, -1.0),
        0.5,
        material_centre,
    ));
    world.push(sphere::Sphere::new(
        glm::vec3(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    world.push(sphere::Sphere::new(
        glm::vec3(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    ));
    world.push(sphere::Sphere::new(
        glm::vec3(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    //* CAMERA
    let look_from = glm::vec3(3.0, 2.0, 2.0);
    let look_at = glm::vec3(0.0, 0.0, -1.0);
    let vup = glm::vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let aperture = 0.0;
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera: Arc<Mutex<camera::Camera>> = Arc::new(Mutex::new(camera::Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    )));

    //* TEXT
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

        let cam: Arc<Mutex<camera::Camera>> = Arc::clone(&camera);
        let (sender, receiver) = channel();
        thread::spawn(move || {
            let mut speed: [f64; 3] = [0.0; 3];
            loop {
            let mouse_speed = receiver.try_recv().unwrap_or([0.0, 0.0]);
            let mut camera = cam.lock().unwrap();
            for i in 0..speed.len() {
                if speed[i].abs() < 0.01 {
                    speed[i] = 0.0;
                }
                if speed[i] > 0.0 {
                    speed[i] -= 0.1;
                } else if speed[i] < 0.0 {
                    speed[i] += 0.1;
                }
            }
            // check for wasd to add or remove speed; make sure to make origin arc mutex
            if camera.wasd[0] {
                speed[0] += 0.2;
            }
            if camera.wasd[1] {
                speed[1] -= 0.2;
            }
            if camera.wasd[2] {
                speed[0] -= 0.2;
            }
            if camera.wasd[3] {
                speed[1] += 0.2;
            }
            if camera.wasd[4] {
                speed[2] += 0.2;
            }
            if camera.wasd[5] {
                speed[2] -= 0.2;
            }
            camera.apply_speed(speed);
            camera.rotate(mouse_speed);
            drop(camera);
            thread::sleep(std::time::Duration::from_millis(20));
        }
    });

    let n_jobs = 12;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(n_jobs).build().unwrap();

    let mut frame_counts: Vec<i32> = vec![0; (WIDTH * HEIGHT) as usize];

    let tile_width = (WIDTH / 4) as usize;

    while let Some(e) = window.next() {
        match e {
            Input(input, _) => match input {
                Button(button_args) => {
                    if let piston_window::Button::Keyboard(key) = button_args.button {
                        let camera = Arc::clone(&camera);
                        let mut camera = camera.lock().unwrap();
                        match button_args.state {
                            piston_window::ButtonState::Press => match key {
                                piston_window::Key::W => {
                                    camera.wasd[0] = true;
                                }
                                piston_window::Key::A => {
                                    camera.wasd[1] = true;
                                }
                                piston_window::Key::S => {
                                    camera.wasd[2] = true;
                                }
                                piston_window::Key::D => {
                                    camera.wasd[3] = true;
                                }
                                piston_window::Key::Space => {
                                    camera.wasd[4] = true;
                                }
                                piston_window::Key::LCtrl => {
                                    camera.wasd[5] = true;
                                }
                                _ => (),
                            },
                            piston_window::ButtonState::Release => match key {
                                piston_window::Key::W => {
                                    camera.wasd[0] = false;
                                }
                                piston_window::Key::A => {
                                    camera.wasd[1] = false;
                                }
                                piston_window::Key::S => {
                                    camera.wasd[2] = false;
                                }
                                piston_window::Key::D => {
                                    camera.wasd[3] = false;
                                }
                                piston_window::Key::Space => {
                                    camera.wasd[4] = false;
                                }
                                piston_window::Key::LCtrl => {
                                    camera.wasd[5] = false;
                                }
                                _ => (),
                            },
                        }
                    }
                }
                piston_window::Input::Move(motion) => {
                    if let piston_window::Motion::MouseRelative(pos) = motion {
                        sender.send(pos).expect("Invalid Mouse Position Recorded!");
                    }
                }
                _ => (),
            },
            Loop(l) => {
                if let piston_window::Loop::Render(_ren) = l {
                    let camera = Arc::clone(&camera);
                    window.draw_2d(&e, |c, g, device| {
                        piston_window::clear([1.0; 4], g);
                        pool.scope(|s| {
                        for i in 0..n_jobs {
                            let now = Instant::now();
                            let mut fbuffer = frame_buffer.clone();
                            let offset_x = (i % 4) * tile_width;
                            let offset_y = (i / 4) * tile_width;
                            let tile_width = if tile_width + offset_y > WIDTH.try_into().unwrap() {
                                WIDTH as usize - offset_y
                            } else {
                                tile_width
                            };
                            s.spawn(move |_| {
                                while now.elapsed().as_millis() <= FRAME_TIME {
                                    let x = (rand::thread_rng().gen_range(0..tile_width) + offset_x) as u32;
                                    let y = (rand::thread_rng().gen_range(0..tile_width) + offset_y).min((HEIGHT - 1).try_into().unwrap()) as u32; 
                                    let pixel = fbuffer.get_pixel_mut(x, y);
                                    let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);

                                    for _i in 0..SAMPLES_PER_PIXEL {
                                        let screen_coords = glm::vec2(
                                            (x as f64 + rand::random::<f64>()) / WIDTH as f64,
                                            1. - ((y as f64 + rand::random::<f64>())
                                                / HEIGHT as f64),
                                        );
                                        let ray: ray::Ray =
                                            camera.lock().unwrap().get_ray(screen_coords);
                                        pixel_color += ray::ray_color(&ray, &world, RAY_DEPTH);
                                    }
                                    *pixel = color::write_pixel(
                                        pixel_color,
                                        *pixel,
                                        SAMPLES_PER_PIXEL,
                                        frame_counts[(x + y * WIDTH) as usize],
                                    );
                                    frame_counts[(x + y * WIDTH) as usize] += 1;
                                }
                            });
                            }
                                
                        });
                        tex.update(&mut tex_context, &frame_buffer).unwrap();
                        piston_window::image(&tex, c.transform, g);
                        tex_context.encoder.flush(device);

                        //* FPS Counter
                        let fps = fps_counter.tick();
                        let fps = ((fps * 10.0).round() / 10.0).to_string();
                        let transform = c.transform.trans(10.0, 30.0);
                        piston_window::text::Text::new(32)
                            .draw(&fps, &mut glyphs, &c.draw_state, transform, g)
                            .unwrap();
                        glyphs.factory.encoder.flush(device);
                    });
                }
            }
            _ => (),
        }
    }
}
