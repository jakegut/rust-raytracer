use std::mem::swap;

use crate::{ray::Ray, vec3::Point};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir[a];
            let mut t0 = (self.min[a] - r.orig[a]) * inv_d;
            let mut t1 = (self.max[a] - r.orig[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn from_surrounding(box0: AABB, box1: AABB) -> AABB {
        let small = Point::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Point::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );

        AABB {
            min: small,
            max: big,
        }
    }
}
