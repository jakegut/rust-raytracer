use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Point,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub mat: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: Arc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return None;
        }
        let sqrtd = disc.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        Some(rec)
    }
}
