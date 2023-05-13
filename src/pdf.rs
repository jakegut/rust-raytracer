use std::{f64::consts::PI, sync::Arc};

use crate::{
    hittable::Hittable,
    object::Object,
    onb::ONB,
    utils::{random_cosine_direction, random_double_normal},
    vec3::{Point, Vec3},
};

pub trait PDF {
    fn value(&self, dir: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        let uvw = ONB::from_w(w);
        Self { uvw }
    }
}

impl PDF for CosinePDF {
    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }

    fn value(&self, dir: &Vec3) -> f64 {
        let cos = dir.unit().dot(self.uvw.w);
        if cos <= 0.0 {
            0.0
        } else {
            cos / PI
        }
    }
}

pub struct HittablePDF {
    o: Point,
    ptr: Arc<Object>,
}

impl HittablePDF {
    pub fn new(o: Point, ptr: Arc<Object>) -> Self {
        Self { o, ptr }
    }
}

impl PDF for HittablePDF {
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }

    fn value(&self, dir: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, dir)
    }
}

pub struct MixturePDF {
    odd: Arc<dyn PDF>,
    even: Arc<dyn PDF>,
}

impl MixturePDF {
    pub fn new(odd: Arc<dyn PDF>, even: Arc<dyn PDF>) -> Self {
        Self { odd, even }
    }
}

impl PDF for MixturePDF {
    fn generate(&self) -> Vec3 {
        if random_double_normal() < 0.5 {
            self.odd.generate()
        } else {
            self.even.generate()
        }
    }

    fn value(&self, dir: &Vec3) -> f64 {
        0.5 * self.odd.value(dir) + 0.5 * self.even.value(dir)
    }
}
