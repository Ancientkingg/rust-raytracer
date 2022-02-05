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



pub fn ray_color(r: &Ray, world: &objects::HittableList) -> glm::TVec3<f64> {

    if let Some(hit) = world.hit(r, 0.001, std::f64::MAX) {
        return 0.5 * (hit.normal + glm::vec3(1.0, 1.0, 1.0));
    }
    let unit_direction = glm::normalize(&r.direction);
    let t = 0.5*(unit_direction.y + 1.0);
    (1.0-t) * glm::vec3(1.0, 1.0, 1.0) + t * glm::vec3(0.5, 0.7, 1.0)
}