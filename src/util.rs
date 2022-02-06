
use nalgebra_glm as glm;
use rand::Rng;
use rand::distributions::Uniform;

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


pub fn near_zero(vector: &glm::TVec3<f64>) -> bool {
    const EPS: f64 = 1.0e-8;
    vector.x.abs() < EPS && vector.y.abs() < EPS && vector.z.abs() < EPS
}

pub fn reflect(vector: &glm::TVec3<f64>, normal: &glm::TVec3<f64>) -> glm::TVec3<f64> {
    *vector - 2.0 * glm::dot(vector, normal) * *normal
}

pub fn refract(vector: &glm::TVec3<f64>, normal: &glm::TVec3<f64>, etai_over_etat: f64) -> glm::TVec3<f64> {
    let cos_theta = glm::dot(&-vector, &normal).min(1.0);
    let r_out_perp = etai_over_etat * (vector + cos_theta*normal);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}