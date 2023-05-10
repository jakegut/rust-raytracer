use crate::{
    aabb::AABB,
    material::{Lambertain, Material},
    ray::Ray,
    vec3::{Point, Vec3},
};
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(Material::Lambertain(Lambertain::default())),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB>;
}
