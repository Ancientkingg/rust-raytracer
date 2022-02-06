use nalgebra_glm as glm;

use crate::objects::Hittable;
use crate::objects;

pub struct Ray {
    pub origin: glm::TVec3<f64>,
    pub direction: glm::TVec3<f64>
}

impl Ray {
    pub fn new(origin: glm::TVec3<f64>, direction: glm::TVec3<f64>) -> Ray {
        Ray {
            origin,
            direction
        }
    }
    pub fn at(&self, t: f64) -> glm::TVec3<f64> {
        self.origin + self.direction * t
    }
}



pub fn ray_color(r: &Ray, world: &objects::HittableList, depth: u8) -> glm::TVec3<f64> {
    if depth <= 0 {
        return glm::vec3(0.0,0.0,0.0);
    }
    if let Some(hit) = world.hit(r, 0.001, std::f64::MAX) {
        if let Some((scattered, attenuation)) = hit.material.scatter(&r, &hit) {
            return attenuation.zip_map(&ray_color(&scattered, &world, depth - 1), |x, y| x * y);
        }
        return glm::vec3(0.0,0.0,0.0);
    }
    let unit_direction = glm::normalize(&r.direction);
    let t = 0.5*(unit_direction.y + 1.0);
    (1.0-t) * glm::vec3(1.0, 1.0, 1.0) + t * glm::vec3(0.5, 0.7, 1.0)
}