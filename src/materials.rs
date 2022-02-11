
use crate::ray;
use crate::objects;
use crate::util;
use nalgebra_glm as glm;
use std::sync::Arc;

pub trait Material: Sync {
    fn scatter(&self, r_in: &ray::Ray, hit: &objects::HitRecord) -> Option<(ray::Ray, glm::TVec3<f64>)>;
}

pub struct Lambertian {
    pub albedo: glm::TVec3<f64>,
}

impl Lambertian {
    pub fn new(albedo: glm::TVec3<f64>) -> Arc<Self> {
        Arc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &ray::Ray, hit: &objects::HitRecord) -> Option<(ray::Ray, glm::TVec3<f64>)> {
        let mut scatter_direction: glm::TVec3<f64> = hit.normal + util::random_unit_vector();
        if util::near_zero(&scatter_direction) {
            scatter_direction = hit.normal;
        }
        let scattered = ray::Ray::new(hit.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: glm::TVec3<f64>,
    pub fuzz: f64,
}

#[allow(dead_code)]
impl Metal {
    pub fn new(albedo: glm::TVec3<f64>, fuzz: f64) -> Arc<Self> {
        Arc::new(Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &ray::Ray, hit: &objects::HitRecord) -> Option<(ray::Ray, glm::TVec3<f64>)> {
        let reflected = util::reflect(&glm::normalize(&r_in.direction), &hit.normal);
        let scattered = ray::Ray::new(hit.p, reflected + self.fuzz * util::random_point_in_sphere());
        if glm::dot(&scattered.direction, &hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

#[allow(dead_code)]
impl Dielectric {
    pub fn new(ir: f64) -> Arc<Self> {
        Arc::new(Dielectric { ir })
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &ray::Ray, hit: &objects::HitRecord) -> Option<(ray::Ray, glm::TVec3<f64>)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = glm::normalize(&r_in.direction);
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let dir: glm::TVec3<f64>;
        if cannot_refract || util::schlick(cos_theta, refraction_ratio) > rand::random() {
            dir = util::reflect(&unit_direction, &hit.normal);
        } else {
            dir = util::refract(&unit_direction, &hit.normal, refraction_ratio);
        }
        let scattered = ray::Ray::new(hit.p, dir);

        Some((scattered, glm::vec3(1.0, 1.0, 1.0)))
    }
}