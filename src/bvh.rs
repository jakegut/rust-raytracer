use core::panic;
use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    obj,
    object::Object,
    ray::Ray,
    utils::{get_all_lights, get_lights_from_node, random_double_normal},
    vec3::{Point, Vec3},
};

pub enum BVHNodeType {
    Leaf(Arc<HittableList>),
    Interior(Arc<BVHNode>, Arc<BVHNode>),
}

pub struct BVHNode {
    pub info: BVHNodeType,
    bx: AABB,
}

impl BVHNode {
    pub fn new(hittable_list: HittableList, time: (f64, f64)) -> Self {
        BVHNode::from_vec(hittable_list.objects, time)
    }

    pub fn from_vec(mut src_objects: Vec<Arc<Object>>, time: (f64, f64)) -> Self {
        let info: BVHNodeType;
        let bx: AABB;

        let mut axis_ranges: Vec<(usize, f64)> =
            (0..3).map(|a| (a, axis_range(&src_objects, a))).collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;
        let cmp = cmp_box(axis);

        let object_span = src_objects.len();

        src_objects.sort_unstable_by(|a, b| cmp(&a, &b));

        match object_span {
            1..=20 => {
                let list = Arc::new(HittableList::from_vec(src_objects));
                info = BVHNodeType::Leaf(list.clone());
                bx = list.bounding_box(time).expect("leaf bounding box");
            }
            _ => {
                let mid = object_span / 2;
                let left = Arc::new(BVHNode::from_vec(src_objects.drain(mid..).collect(), time));
                let right = Arc::new(BVHNode::from_vec(src_objects, time));

                let box_left = left.bounding_box(time).expect("left bounding box");
                let box_right = right.bounding_box(time).expect("right bounding box");

                bx = AABB::from_surrounding(box_left, box_right);
                info = BVHNodeType::Interior(left, right);
            }
        };

        Self { info, bx }
    }

    pub fn get_lights(&self) -> Vec<Arc<Object>> {
        match &self.info {
            BVHNodeType::Interior(left, right) => get_lights_from_node(left.clone(), right.clone()),
            BVHNodeType::Leaf(hl) => hl.get_lights(),
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

        match &self.info {
            BVHNodeType::Leaf(hl) => hl.hit(r, t_min, t_max),
            BVHNodeType::Interior(left, right) => {
                let hit_left = left.hit(r, t_min, t_max);

                let t = if let Some(hc) = &hit_left {
                    hc.t
                } else {
                    t_max
                };

                let hit_right = right.hit(r, t_min, t);

                hit_right.or(hit_left)
            }
        }
    }

    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        if !self.bx.hit(&Ray::new(*o, *v), 0.0001, f64::MAX) {
            return 0.0;
        }

        match &self.info {
            BVHNodeType::Interior(left, right) => {
                0.5 * left.pdf_value(o, v) + 0.5 * right.pdf_value(o, v)
            }
            BVHNodeType::Leaf(hl) => hl.pdf_value(o, v),
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        match &self.info {
            BVHNodeType::Interior(left, right) => {
                if random_double_normal() < 0.5 {
                    left.random(o)
                } else {
                    right.random(o)
                }
            }
            BVHNodeType::Leaf(hl) => hl.random(o),
        }
    }
}

fn cmp_box(axis: usize) -> impl Fn(&Object, &Object) -> Ordering {
    move |a, b| {
        let box_a = a.bounding_box((0.0, 0.0)).expect("some bounding box");
        let box_b = b.bounding_box((0.0, 0.0)).expect("some bounding box");
        let ac = box_a.min[axis] + box_a.max[axis];
        let bc = box_b.min[axis] + box_b.max[axis];
        return ac.partial_cmp(&bc).unwrap();
    }
}

fn axis_range(objects: &Vec<Arc<Object>>, axis: usize) -> f64 {
    let (min, max) = objects
        .iter()
        .fold((f64::MAX, f64::MIN), |(bmin, bmax), hit| {
            if let Some(aabb) = hit.bounding_box((0.0, 0.0)) {
                (bmin.min(aabb.min[axis]), bmax.max(aabb.max[axis]))
            } else {
                (bmin, bmax)
            }
        });
    max - min
}
