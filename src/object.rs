use crate::{
    aabb::AABB,
    bvh::BVHNode,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    sphere::{MovingSphere, Sphere},
};

pub enum Object {
    Sphere(Sphere),
    HittableList(HittableList),
    MovingSphere(MovingSphere),
    BVHNode(BVHNode),
}

impl Hittable for Object {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(s) => s.hit(r, t_min, t_max),
            Object::HittableList(hl) => hl.hit(r, t_min, t_max),
            Object::MovingSphere(ms) => ms.hit(r, t_min, t_max),
            Object::BVHNode(node) => node.hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        match self {
            Object::Sphere(s) => s.bounding_box(time),
            Object::HittableList(hl) => hl.bounding_box(time),
            Object::MovingSphere(ms) => ms.bounding_box(time),
            Object::BVHNode(node) => node.bounding_box(time),
        }
    }
}
