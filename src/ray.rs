use crate::vec3::{Point, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Ray {
        Ray {
            orig,
            dir,
            time: 0.0,
        }
    }

    pub fn with_time(&mut self, time: f64) -> Self {
        self.time = time;
        *self
    }

    pub fn at(self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
