use crate::{
    aabb::AABB,
    material::{Lambertain, Material},
    object::Object,
    pdf::PDF,
    ray::Ray,
    vec3::{Color, Point, Vec3},
};
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub uv: (f64, f64),
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
            uv: (0.0, 0.0),
            mat: Arc::new(Material::Lambertain(Lambertain::default())),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB>;
    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        return 0.0;
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct FlipFace {
    pub ptr: Arc<Object>,
}

impl FlipFace {
    pub fn new(ptr: Arc<Object>) -> Self {
        Self { ptr }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.ptr.hit(r, t_min, t_max) {
            Some(mut rec) => {
                rec.front_face = !rec.front_face;
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        self.ptr.bounding_box(time)
    }

    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        self.ptr.pdf_value(o, v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.ptr.random(o)
    }
}
