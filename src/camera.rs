
use crate::ray;
use nalgebra_glm as glm;

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    origin: glm::TVec3<f64>,
    horizontal: glm::TVec3<f64>,
    vertical: glm::TVec3<f64>,
    lower_left_corner: glm::TVec3<f64>,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin: glm::TVec3<f64> = glm::vec3(0.0, 0.0, 0.0);
        let horizontal: glm::TVec3<f64> = glm::vec3(viewport_width, 0.0, 0.0);
        let vertical: glm::TVec3<f64> =glm::vec3(0.0, viewport_height, 0.0);
        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,

            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - glm::vec3(0.0, 0.0, focal_length),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, screen_coords: glm::TVec2<f64>) -> ray::Ray {
        ray::Ray::new(self.origin, self.lower_left_corner + self.horizontal * screen_coords.x + self.vertical * screen_coords.y - self.origin)
    }
}