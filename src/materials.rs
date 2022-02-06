
use crate::ray;
use crate::objects;
use nalgebra_glm as glm;

pub trait Material {
    fn scatter(&self, r_in: &ray::Ray, hit: &objects::HitRecord, attenuation: &glm::TVec3<f64>, scattered: &ray::Ray) -> Option<(ray::Ray, glm::TVec3<f64>)>;
}