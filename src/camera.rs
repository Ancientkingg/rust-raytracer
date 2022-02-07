
use crate::ray;
use crate::util;
use nalgebra_glm as glm;

#[allow(dead_code)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub vfov: f64,
    origin: glm::TVec3<f64>,
    horizontal: glm::TVec3<f64>,
    vertical: glm::TVec3<f64>,
    lower_left_corner: glm::TVec3<f64>,
    u: glm::TVec3<f64>,
    v: glm::TVec3<f64>,
    w: glm::TVec3<f64>,
    lens_radius: f64,
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
        }
    }
}


impl Camera {
    pub fn get_ray(&self, screen_coords: glm::TVec2<f64>) -> ray::Ray {
        let rd = self.lens_radius * util::random_point_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        ray::Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * screen_coords.x + self.vertical * screen_coords.y - self.origin - offset)
    }
}