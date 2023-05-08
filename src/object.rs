use crate::{hittable::Hittable, hittable_list::HittableList, sphere::Sphere};

pub enum Object {
    Sphere(Sphere),
    HittableList(HittableList),
}

impl Hittable for Object {
    fn hit(
        &self,
        r: crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        match self {
            Object::Sphere(s) => s.hit(r, t_min, t_max, rec),
            Object::HittableList(hl) => hl.hit(r, t_min, t_max, rec),
        }
    }
}
