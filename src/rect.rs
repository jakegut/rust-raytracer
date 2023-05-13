use std::sync::Arc;

use eframe::egui::accesskit::Rect;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::Material,
    object::Object,
    ray::Ray,
    texture::Texture,
    utils::random_double,
    vec3::{Point, Vec3},
};

pub struct XYRect {
    x: (f64, f64),
    y: (f64, f64),
    k: f64,
    pub mat: Arc<Material>,
}

impl XYRect {
    pub fn new(x: (f64, f64), y: (f64, f64), k: f64, mat: Arc<Material>) -> Self {
        Self { x, y, k, mat }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self, _time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(
            Point::new(self.x.0, self.y.0, self.k - 0.0001),
            Point::new(self.x.1, self.y.1, self.k + 0.0001),
        ))
    }

    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        };

        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;

        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }
        let mut rec = HitRecord::default();
        let u = (x - self.x.0) / (self.x.1 - self.x.0);
        let v = (y - self.y.0) / (self.y.1 - self.y.0);
        rec.t = t;
        rec.uv = (u, v);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        rec.p = r.at(t);

        Some(rec)
    }
}

pub struct XZRect {
    x: (f64, f64),
    z: (f64, f64),
    k: f64,
    pub mat: Arc<Material>,
}

impl XZRect {
    pub fn new(x: (f64, f64), z: (f64, f64), k: f64, mat: Arc<Material>) -> Self {
        Self { x, z, k, mat }
    }
}

impl Hittable for XZRect {
    fn bounding_box(&self, _time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x.0, self.k - 0.0001, self.z.0),
            Vec3::new(self.x.1, self.k + 0.0001, self.z.1),
        ))
    }
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;

        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }

        let mut rec = HitRecord::default();

        let u = (x - self.x.0) / (self.x.1 - self.x.0);
        let v = (z - self.z.0) / (self.z.1 - self.z.0);

        rec.t = t;
        rec.uv = (u, v);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        rec.p = r.at(t);
        Some(rec)
    }
    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        match self.hit(&Ray::new(*o, *v), 0.001, f64::MAX) {
            None => 0.0,
            Some(rec) => {
                let area = (self.x.1 - self.x.0) * (self.z.1 - self.z.0);
                let dist_sqrd = rec.t * rec.t * v.length_squared();
                let cos = (v.dot(rec.normal)).abs() / v.length();

                dist_sqrd / (cos * area)
            }
        }
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Point::new(
            random_double(self.x.0, self.x.1),
            self.k,
            random_double(self.z.0, self.z.1),
        );
        random_point - *o
    }
}

pub struct YZRect {
    y: (f64, f64),
    z: (f64, f64),
    k: f64,
    pub mat: Arc<Material>,
}

impl YZRect {
    pub fn new(y: (f64, f64), z: (f64, f64), k: f64, mat: Arc<Material>) -> Self {
        Self { y, z, k, mat }
    }
}

impl Hittable for YZRect {
    fn bounding_box(&self, _time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(
            Point::new(self.k - 0.0001, self.y.0, self.z.0),
            Point::new(self.k + 0.0001, self.y.1, self.z.1),
        ))
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;

        if y < self.y.0 || y > self.y.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }

        let mut rec = HitRecord::default();
        rec.t = t;
        let u = (y - self.y.0) / (self.y.1 - self.y.0);
        let v = (z - self.z.0) / (self.z.1 - self.z.0);
        rec.uv = (u, v);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        rec.p = r.at(t);
        Some(rec)
    }
}

pub struct RectBox {
    box_min: Point,
    box_max: Point,
    sides: HittableList,
}

impl RectBox {
    pub fn new(p0: &Point, p1: &Point, mat: Arc<Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(Object::XYRect(XYRect::new(
            (p0.x, p1.x),
            (p0.y, p1.y),
            p1.z,
            mat.clone(),
        ))));
        sides.add(Arc::new(Object::XYRect(XYRect::new(
            (p0.x, p1.x),
            (p0.y, p1.y),
            p0.z,
            mat.clone(),
        ))));

        sides.add(Arc::new(Object::XZRect(XZRect::new(
            (p0.x, p1.x),
            (p0.z, p1.z),
            p1.y,
            mat.clone(),
        ))));
        sides.add(Arc::new(Object::XZRect(XZRect::new(
            (p0.x, p1.x),
            (p0.z, p1.z),
            p0.y,
            mat.clone(),
        ))));

        sides.add(Arc::new(Object::YZRect(YZRect::new(
            (p0.y, p1.y),
            (p0.z, p1.z),
            p1.x,
            mat.clone(),
        ))));
        sides.add(Arc::new(Object::YZRect(YZRect::new(
            (p0.y, p1.y),
            (p0.z, p1.z),
            p0.x,
            mat.clone(),
        ))));

        Self {
            box_min: *p0,
            box_max: *p1,
            sides,
        }
    }
}

impl Hittable for RectBox {
    fn bounding_box(&self, _time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
