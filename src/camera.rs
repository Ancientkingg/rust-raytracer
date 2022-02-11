
use crate::ray;
use crate::util;
use nalgebra_glm as glm;

#[allow(dead_code)] #[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub vfov: f64,
    pub wasd: [bool; 6],
    origin: glm::TVec3<f64>,
    horizontal: glm::TVec3<f64>,
    vertical: glm::TVec3<f64>,
    lower_left_corner: glm::TVec3<f64>,
    u: glm::TVec3<f64>,
    v: glm::TVec3<f64>,
    w: glm::TVec3<f64>,
    lens_radius: f64,
    focus_dist: f64
}

impl Camera {
    pub fn new(lookfrom: glm::TVec3<f64>, lookat: glm::TVec3<f64>, vup: glm::TVec3<f64>, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = util::degrees_to_radians(vfov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = glm::normalize(&(lookfrom - lookat));
        let u = glm::normalize(&glm::cross(&vup, &w));
        let v = glm::cross(&w, &u);

        let origin: glm::TVec3<f64> = lookfrom;
        let horizontal: glm::TVec3<f64> = focus_dist * viewport_width * u;
        let vertical: glm::TVec3<f64> = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;
        Camera {
            aspect_ratio,
            vfov,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,v,w,
            lens_radius,
            wasd: [false; 6],
            focus_dist
        }
    }
}


impl Camera {
    pub fn get_ray(&self, screen_coords: glm::TVec2<f64>) -> ray::Ray {
        let rd = self.lens_radius * util::random_point_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        ray::Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * screen_coords.x + self.vertical * screen_coords.y - self.origin - offset)
    }
    pub fn apply_speed(&mut self, speed: [f64; 3]) {
        self.origin += self.w * -speed[0] * 0.1;
        self.origin += self.u * speed[1] * 0.1;
        self.origin += self.v * speed[2] * 0.1;
        self.lower_left_corner = self.origin - self.horizontal/2.0 - self.vertical/2.0 - self.focus_dist * self.w;
    }
    pub fn rotate(&mut self, mouse_speed: [f64; 2]) {
        let theta = util::degrees_to_radians(self.vfov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = self.aspect_ratio * viewport_height;
        let horizontal_movement = -mouse_speed[0] * 0.0005;
        let vertical_movement = -mouse_speed[1] * 0.0005;
        self.w = glm::rotate_y_vec3(&self.w, horizontal_movement);
        self.u = glm::rotate_y_vec3(&self.u, horizontal_movement);
        self.v = glm::rotate_y_vec3(&self.v, horizontal_movement);

        self.w = glm::rotate_vec3(&self.w, vertical_movement, &self.u);
        self.u = glm::rotate_vec3(&self.u, vertical_movement, &self.u);
        self.v = glm::rotate_vec3(&self.v, vertical_movement, &self.u);

        self.horizontal = self.focus_dist * viewport_width * self.u;
        self.vertical = self.focus_dist * viewport_height * self.v;
        self.lower_left_corner = self.origin - self.horizontal/2.0 - self.vertical/2.0 - self.focus_dist * self.w;
    }
}