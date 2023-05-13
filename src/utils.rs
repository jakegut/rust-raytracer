use std::f64::consts::PI;

use rand::Rng;

use crate::vec3::Vec3;

pub fn random_double_normal() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double_normal();
    let r2 = random_double_normal();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
