use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::{Material, ScatterRecord, Scatterable},
    obj::load_obj,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct TriangleMesh {
    pub verts: Vec<Vec3>,
    pub faces: Vec<(u32, u32, u32)>,
    pub mat: Arc<Material>,
    bx: AABB,
}

impl TriangleMesh {
    pub fn new(path: String, mat: Arc<Material>) -> Self {
        let obj = load_obj(path).expect("invalid obj");

        let mut vmin = obj.ver_array[0];
        let mut vmax = obj.ver_array[1];
        for v in obj.ver_array[1..].to_vec() {
            vmin = Vec3 {
                x: vmin.x.min(v.x),
                y: vmin.y.min(v.y),
                z: vmin.z.min(v.z),
            };
            vmax = Vec3 {
                x: vmax.x.max(v.x),
                y: vmax.y.max(v.y),
                z: vmax.z.max(v.z),
            };
        }

        let bx = AABB::new(vmin, vmax);

        TriangleMesh {
            verts: obj.ver_array,
            faces: obj.face_array,
            mat,
            bx,
        }
    }
}

impl Hittable for TriangleMesh {
    fn bounding_box(&self, time: (f64, f64)) -> Option<crate::aabb::AABB> {
        Some(self.bx)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bx.hit(r, t_min, t_max) {
            return None;
        }

        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for face in &self.faces {
            let v0 = self.verts[face.0 as usize - 1];
            let v1 = self.verts[face.1 as usize - 1];
            let v2 = self.verts[face.2 as usize - 1];

            match triangle_intersect(r, (&v0, &v1, &v2)) {
                Some((t, uv, normal)) => {
                    if t < closest_so_far && t > t_min {
                        let mut new_rec = HitRecord::default();
                        new_rec.p = r.at(t);
                        new_rec.set_face_normal(r, normal);
                        new_rec.t = t;
                        new_rec.uv = uv;
                        new_rec.mat = self.mat.clone();
                        rec = Some(new_rec);
                        closest_so_far = t;
                    }
                }
                _ => continue,
            };
        }

        rec
    }
}

pub fn triangle_intersect(r: &Ray, v: (&Point, &Point, &Point)) -> Option<(f64, (f64, f64), Vec3)> {
    let v0v1 = *v.1 - v.0;
    let v0v2 = *v.2 - v.0;
    let pvec = r.dir.cross(v0v2);
    let det = v0v1.dot(pvec);

    if det.abs() < f64::EPSILON {
        return None;
    }

    let inv_det = 1.0 / det;

    let t_vec = r.orig - v.0;
    let u = t_vec.dot(pvec) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q_vec = t_vec.cross(v0v1);
    let v = r.dir.dot(q_vec) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = v0v2.dot(q_vec) * inv_det;

    let mut normal = v0v1.cross(v0v2).unit();

    // if det < 0.0 {
    //     normal = -normal;
    // }

    Some((t, (u, v), normal))
}
