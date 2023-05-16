use std::{f64::consts::PI, sync::Arc, vec};

use rand::Rng;

use crate::{
    bvh::{BVHNode, BVHNodeType},
    object::Object,
    vec3::Vec3,
};

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

pub fn get_all_lights(v: &Vec<Arc<Object>>) -> Vec<Arc<Object>> {
    v.iter()
        .flat_map(|o| {
            if o.is_light() {
                vec![o.clone()]
            } else {
                o.get_lights()
            }
        })
        .clone()
        .collect()
}

pub fn get_lights_from_node(left: Arc<BVHNode>, right: Arc<BVHNode>) -> Vec<Arc<Object>> {
    vec![left, right]
        .iter()
        .flat_map(|o| match &o.info {
            BVHNodeType::Interior(left, right) => get_lights_from_node(left.clone(), right.clone()),
            BVHNodeType::Leaf(hl) => hl.get_lights(),
        })
        .clone()
        .collect()
}
