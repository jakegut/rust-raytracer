use glam::DVec3;

use crate::{
    aabb::AABB,
    material::{Lambertain, Material},
    object::{self, Object},
    pdf::PDF,
    ray::Ray,
    vec3::{Color, Point, Vec3},
};
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub uv: (f64, f64),
    pub front_face: bool,
    pub mat: Arc<Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            uv: (0.0, 0.0),
            mat: Arc::new(Material::Lambertain(Lambertain::default())),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB>;
    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        return 0.0;
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct FlipFace {
    pub ptr: Arc<Object>,
}

impl FlipFace {
    pub fn new(ptr: Arc<Object>) -> Self {
        Self { ptr }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.ptr.hit(r, t_min, t_max) {
            Some(mut rec) => {
                rec.front_face = !rec.front_face;
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        self.ptr.bounding_box(time)
    }

    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        self.ptr.pdf_value(o, v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.ptr.random(o)
    }
}

pub struct MatTransform {
    pub mat_i: glam::DMat4,
    pub ptr: Arc<Object>,
}

impl MatTransform {
    pub fn new(mat: glam::DMat4, ptr: Arc<Object>) -> Self {
        Self {
            mat_i: mat.inverse(),
            ptr,
        }
    }
}

impl Hittable for MatTransform {
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        let aabb = self.ptr.bounding_box(time);

        match aabb {
            None => None,
            Some(aabb) => {
                let mat = self.mat_i.inverse();

                Some(AABB {
                    min: mat.transform_point3(aabb.min.into()).into(),
                    max: mat.transform_point3(aabb.max.into()).into(),
                })
            }
        }
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let osd = self.mat_i.transform_vector3(r.dir.into());

        let oso = self.mat_i.transform_point3(r.orig.into());

        let object_space_ray = Ray::new(
            Vec3::new(oso.x, oso.y, oso.z),
            Vec3::new(osd.x, osd.y, osd.z),
        );

        match self.ptr.hit(&object_space_ray, t_min, t_max) {
            None => None,
            Some(rec) => {
                let world_space_mat = self.mat_i.transpose();
                let world_normal =
                    Vec3::from(world_space_mat.transform_vector3(rec.normal.into())).unit();
                let world_p = Vec3::from(self.mat_i.inverse().transform_point3(rec.p.into()));

                Some(HitRecord {
                    p: world_p,
                    normal: world_normal,
                    t: rec.t,
                    uv: rec.uv,
                    front_face: rec.front_face,
                    mat: rec.mat,
                })
            }
        }
    }

    fn pdf_value(&self, o: &Point, v: &Point) -> f64 {
        let object_o = Vec3::from(self.mat_i.transform_point3((*o).into()));
        let object_v = Vec3::from(self.mat_i.transform_point3((*v).into()));

        self.ptr.pdf_value(&object_o, &object_v)
        // self.ptr.pdf_value(o, v)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let object_o = Vec3::from(self.mat_i.transform_vector3((*o).into()));
        Vec3::from(
            self.mat_i
                .inverse()
                .transform_vector3(self.ptr.random(&object_o).into()),
        )
        // self.ptr.random(o)
    }
}
