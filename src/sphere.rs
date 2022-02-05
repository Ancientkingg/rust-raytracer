
use nalgebra_glm as glm;
use crate::ray;
use crate::objects;

use objects::Hittable;

use self::objects::HitRecord;

pub struct Sphere {
    pub centre: glm::TVec3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: glm::TVec3<f64>, radius: f64) -> Sphere {
        Sphere {
            centre,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<objects::HitRecord> {
        let oc = r.origin - self.centre;
        let a = glm::dot(&r.direction, &r.direction);
        let b = glm::dot(&oc, &r.direction);
        let c = glm::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.centre) / self.radius;
                return Some(HitRecord { t, p, normal })
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.centre) / self.radius;
                return Some(HitRecord { t, p, normal })
            }
        }
        None
    }
}

