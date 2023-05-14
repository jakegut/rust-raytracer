use std::{sync::Arc, vec};

use crate::{
    aabb::AABB,
    bvh::BVHNode,
    hittable::{FlipFace, HitRecord, Hittable, MatTransform},
    hittable_list::HittableList,
    material::Material,
    mesh::{Triangle, TriangleMesh},
    rect::{RectBox, XYRect, XZRect, YZRect},
    sphere::{MovingSphere, Sphere},
};

pub enum Object {
    Sphere(Sphere),
    HittableList(HittableList),
    MovingSphere(MovingSphere),
    BVHNode(BVHNode),
    XYRect(XYRect),
    YZRect(YZRect),
    XZRect(XZRect),
    RectBox(RectBox),
    TriangleMesh(TriangleMesh),
    Triangle(Triangle),

    FlipFace(FlipFace),
    MatTransform(MatTransform),
}

impl Default for Object {
    fn default() -> Self {
        Self::HittableList(HittableList::new())
    }
}

impl Object {
    pub fn get_lights(&self) -> Vec<Arc<Object>> {
        match self {
            Object::BVHNode(n) => n.get_lights(),
            Object::HittableList(hl) => hl.get_lights(),
            _ => {
                vec![]
            }
        }
    }

    pub fn is_light(&self) -> bool {
        match self {
            Object::FlipFace(ff) => ff.ptr.is_light(),
            Object::MatTransform(mt) => mt.ptr.is_light(),
            _ => {
                let mat = match self {
                    Object::XZRect(r) => Some(r.mat.clone()),
                    Object::XYRect(r) => Some(r.mat.clone()),
                    Object::Sphere(r) => Some(r.mat.clone()),
                    _ => None,
                };

                match mat {
                    Some(m) => match *m {
                        Material::DiffuseLight(_) => true,
                        Material::Dielectric(_) => true,
                        _ => false,
                    },
                    None => false,
                }
            }
        }
    }
}

impl Hittable for Object {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(s) => s.hit(r, t_min, t_max),
            Object::HittableList(hl) => hl.hit(r, t_min, t_max),
            Object::MovingSphere(ms) => ms.hit(r, t_min, t_max),
            Object::BVHNode(node) => node.hit(r, t_min, t_max),
            Object::XYRect(rect) => rect.hit(r, t_min, t_max),
            Object::YZRect(rect) => rect.hit(r, t_min, t_max),
            Object::XZRect(rect) => rect.hit(r, t_min, t_max),
            Object::RectBox(rect) => rect.hit(r, t_min, t_max),
            Object::FlipFace(ff) => ff.hit(r, t_min, t_max),
            Object::MatTransform(mt) => mt.hit(r, t_min, t_max),
            Object::TriangleMesh(tm) => tm.hit(r, t_min, t_max),
            Object::Triangle(tm) => tm.hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        match self {
            Object::Sphere(s) => s.bounding_box(time),
            Object::HittableList(hl) => hl.bounding_box(time),
            Object::MovingSphere(ms) => ms.bounding_box(time),
            Object::BVHNode(node) => node.bounding_box(time),
            Object::XYRect(rect) => rect.bounding_box(time),
            Object::YZRect(rect) => rect.bounding_box(time),
            Object::XZRect(rect) => rect.bounding_box(time),
            Object::RectBox(rect) => rect.bounding_box(time),
            Object::FlipFace(ff) => ff.bounding_box(time),
            Object::MatTransform(mt) => mt.bounding_box(time),
            Object::TriangleMesh(tm) => tm.bounding_box(time),
            Object::Triangle(tm) => tm.bounding_box(time),
        }
    }

    fn pdf_value(&self, o: &crate::vec3::Point, v: &crate::vec3::Point) -> f64 {
        match self {
            Object::Sphere(s) => s.pdf_value(o, v),
            Object::HittableList(hl) => hl.pdf_value(o, v),
            Object::MovingSphere(ms) => ms.pdf_value(o, v),
            Object::BVHNode(node) => node.pdf_value(o, v),
            Object::XYRect(rect) => rect.pdf_value(o, v),
            Object::YZRect(rect) => rect.pdf_value(o, v),
            Object::XZRect(rect) => rect.pdf_value(o, v),
            Object::RectBox(rect) => rect.pdf_value(o, v),
            Object::FlipFace(ff) => ff.pdf_value(o, v),
            Object::MatTransform(mt) => mt.pdf_value(o, v),
            Object::TriangleMesh(mt) => mt.pdf_value(o, v),
            Object::Triangle(mt) => mt.pdf_value(o, v),
        }
    }

    fn random(&self, o: &crate::vec3::Vec3) -> crate::vec3::Vec3 {
        match self {
            Object::Sphere(s) => s.random(o),
            Object::HittableList(hl) => hl.random(o),
            Object::MovingSphere(ms) => ms.random(o),
            Object::BVHNode(node) => node.random(o),
            Object::XYRect(rect) => rect.random(o),
            Object::YZRect(rect) => rect.random(o),
            Object::XZRect(rect) => rect.random(o),
            Object::RectBox(rect) => rect.random(o),
            Object::FlipFace(ff) => ff.random(o),
            Object::MatTransform(mt) => mt.random(o),
            Object::TriangleMesh(mt) => mt.random(o),
            Object::Triangle(mt) => mt.random(o),
        }
    }
}
