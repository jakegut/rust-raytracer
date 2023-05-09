use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    sphere::{MovingSphere, Sphere},
};

pub enum Object {
    Sphere(Sphere),
    HittableList(HittableList),
    MovingSphere(MovingSphere),
}

impl Hittable for Object {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(s) => s.hit(r, t_min, t_max),
            Object::HittableList(hl) => hl.hit(r, t_min, t_max),
            Object::MovingSphere(ms) => ms.hit(r, t_min, t_max),
        }
    }
}
