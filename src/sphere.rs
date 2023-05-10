use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
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

    fn bounding_box(&self, time: (f64, f64)) -> Option<crate::aabb::AABB> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(
            self.center - radius_vec,
            self.center + radius_vec,
        ))
    }
}

pub struct MovingSphere {
    center: (Point, Point),
    time: (f64, f64),
    radius: f64,
    mat: Arc<Material>,
}

impl MovingSphere {
    pub fn new(center: (Point, Point), time: (f64, f64), radius: f64, mat: Arc<Material>) -> Self {
        MovingSphere {
            center,
            time,
            radius,
            mat,
        }
    }

    pub fn center(&self, time: f64) -> Point {
        self.center.0
            + ((time - self.time.0) / (self.time.1 - self.time.0)) * (self.center.1 - self.center.0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.time);
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
        let outward_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        Some(rec)
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(
            self.center(time.0) - radius_vec,
            self.center(time.0) + radius_vec,
        );
        let box1 = AABB::new(
            self.center(time.1) - radius_vec,
            self.center(time.1) + radius_vec,
        );
        Some(AABB::from_surrounding(box0, box1))
    }
}
