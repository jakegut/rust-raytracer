use core::panic;
use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    object::Object,
    utils::get_all_lights,
    vec3::{Point, Vec3},
};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<Object>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn get_lights(&self) -> Vec<Arc<Object>> {
        get_all_lights(&self.objects)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.objects.len() == 0 {
            return None;
        }

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

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        if self.objects.len() == 0 {
            return None;
        };

        let mut output_box = AABB::default();
        let mut first_box = true;

        for object in self.objects.iter() {
            match object.bounding_box(time) {
                None => return None,
                Some(aabb) => {
                    output_box = if first_box {
                        aabb
                    } else {
                        AABB::from_surrounding(output_box, aabb)
                    };
                    first_box = false
                }
            }
        }

        Some(output_box)
    }

    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        let wt = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in self.objects.iter() {
            sum += wt * object.pdf_value(o, v)
        }
        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        if self.objects.len() == 0 {
            return Vec3::random_unit_vector();
        }
        let mut rng = thread_rng();
        let size = self.objects.len();
        let r: usize = rng.gen_range(0..size);
        self.objects[r].random(o)
    }
}
