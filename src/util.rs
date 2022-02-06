
use nalgebra_glm as glm;
use rand::Rng;
use rand::distributions::{Uniform};

pub fn random_point_in_sphere() -> glm::TVec3<f64> {
    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(-1.0..=1.0);
    loop {
        let x = rng.sample(&distribution);
        let y = rng.sample(&distribution);
        let z = rng.sample(&distribution);
        let vector = glm::vec3(x, y, z);
        if glm::dot(&vector, &vector) <= 1.0 {
            return vector;
        }
    }
}

pub fn random_unit_vector() -> glm::TVec3<f64> {
    glm::normalize(&random_point_in_sphere())
}