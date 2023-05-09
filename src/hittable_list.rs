use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    object::Object,
};

pub struct HittableList {
    pub objects: Vec<Arc<Object>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<Object>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let obj = object.as_ref();
            match obj.hit(r, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    rec = Some(hit)
                }
                None => continue,
            }
        }

        return rec;
    }
}
