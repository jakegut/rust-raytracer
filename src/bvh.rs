use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    object::Object,
    ray::Ray,
};

pub struct BVHNode {
    left: Arc<Object>,
    right: Arc<Object>,
    bx: AABB,
}

impl BVHNode {
    pub fn new(hittable_list: &mut HittableList, time: (f64, f64)) -> Self {
        let len = hittable_list.objects.len();
        BVHNode::from_vec(&mut hittable_list.objects, 0, len, time)
    }

    pub fn from_vec(
        src_objects: &mut [Arc<Object>],
        start: usize,
        end: usize,
        time: (f64, f64),
    ) -> Self {
        let left: Arc<Object>;
        let right: Arc<Object>;

        let mut rng = rand::thread_rng();
        let objects = src_objects;
        let axis: usize = rng.gen_range(0..=2);
        let cmp = cmp_box(axis);

        let object_span = end - start;

        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => match cmp(&objects[start], &objects[start + 1]) {
                Ordering::Less => {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                }
                _ => {
                    right = objects[start].clone();
                    left = objects[start + 1].clone();
                }
            },
            _ => {
                objects.sort_by(|a, b| cmp(&a, &b));
                let mid = start + object_span / 2;
                left = Arc::new(Object::BVHNode(BVHNode::from_vec(
                    objects, start, mid, time,
                )));
                right = Arc::new(Object::BVHNode(BVHNode::from_vec(objects, mid, end, time)));
            }
        };

        let box_left = left.bounding_box(time);
        let box_right = right.bounding_box(time);

        if box_left.is_none() && box_right.is_none() {
            panic!()
        }

        let bxs = Some((box_left, box_right));

        match bxs {
            Some((Some(bx_left), Some(bx_right))) => Self {
                left,
                right,
                bx: AABB::from_surrounding(bx_left, bx_right),
            },
            _ => unreachable!(),
        }
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self, _time: (f64, f64)) -> Option<AABB> {
        Some(self.bx)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bx.hit(r, t_min, t_max) {
            return None;
        };

        let hit_left = self.left.hit(r, t_min, t_max);

        let lhc = hit_left.clone();
        let t = if let Some(hc) = hit_left { hc.t } else { t_max };

        let hit_right = self.right.hit(r, t_min, t);

        hit_right.or(lhc)
    }
}

fn cmp_box(axis: usize) -> impl Fn(&Object, &Object) -> Ordering {
    move |a, b| {
        let box_a = a.bounding_box((0.0, 0.0));
        let box_b = b.bounding_box((0.0, 0.0));
        if let Some(box_a) = box_a {
            if let Some(box_b) = box_b {
                return box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}
