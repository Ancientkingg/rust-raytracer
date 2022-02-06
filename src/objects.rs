
use nalgebra_glm as glm;
use crate::ray;
use crate::materials;

#[allow(dead_code)]
pub struct HitRecord<'a> {
    pub p: glm::TVec3<f64>,
    pub normal: glm::TVec3<f64>,
    pub t: f64,
    pub material: &'a dyn materials::Material,
    pub front_face: bool
}

pub fn set_face_normal(r: &ray::Ray, outward_normal: glm::TVec3<f64>) -> (glm::TVec3<f64>, bool) {
    let front_face = glm::dot(&r.direction, &outward_normal) < 0.0;
    let normal: glm::TVec3<f64> = if front_face {
        outward_normal
    } else {
        (-1.0) * outward_normal
    };
    (normal, front_face)
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: std::vec::Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Box::new(hittable));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}