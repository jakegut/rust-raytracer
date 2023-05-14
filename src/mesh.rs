use std::sync::Arc;

use crate::{
    aabb::AABB,
    bvh::BVHNode,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::Material,
    obj::load_obj,
    object::Object,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Triangle {
    pub vs: (Vec3, Vec3, Vec3),
}

impl Hittable for Triangle {
    fn bounding_box(&self, _time: (f64, f64)) -> Option<AABB> {
        let min = Vec3::new(
            self.vs.0.x.min(self.vs.1.x).min(self.vs.2.x),
            self.vs.0.y.min(self.vs.1.y).min(self.vs.2.y),
            self.vs.0.z.min(self.vs.1.z).min(self.vs.2.z),
        );
        let max = Vec3::new(
            self.vs.0.x.max(self.vs.1.x).max(self.vs.2.x),
            self.vs.0.y.max(self.vs.1.y).max(self.vs.2.y),
            self.vs.0.z.max(self.vs.1.z).max(self.vs.2.z),
        );
        Some(AABB { min, max })
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match triangle_intersect(r, self.vs) {
            Some((t, uv, normal)) => {
                if t < t_max && t > t_min {
                    let mut new_rec = HitRecord::default();
                    new_rec.p = r.at(t);
                    new_rec.set_face_normal(r, normal);
                    new_rec.t = t;
                    new_rec.uv = uv;
                    Some(new_rec)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub struct Mesh {
    // pub verts: HittableList,
    pub node: BVHNode,
    pub bx: AABB,
}

impl Mesh {
    pub fn new(path: String) -> Self {
        let obj = load_obj(path).expect("invalid obj");

        let mut tris = HittableList::new();

        for face in &obj.face_array {
            let v0 = obj.ver_array[face.0 as usize - 1];
            let v1 = obj.ver_array[face.1 as usize - 1];
            let v2 = obj.ver_array[face.2 as usize - 1];

            tris.add(Arc::new(Object::Triangle(Triangle { vs: (v0, v1, v2) })))
        }

        let node = BVHNode::new(&mut tris, (0.0, 0.0));

        let mut vmin = obj.ver_array[0];
        let mut vmax = obj.ver_array[0];
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

        Self { node, bx }
    }
}

pub struct TriangleMesh {
    pub mesh: Arc<Mesh>,
    pub mat: Arc<Material>,
}

impl TriangleMesh {
    pub fn new(mesh: Arc<Mesh>, mat: Arc<Material>) -> Self {
        TriangleMesh { mesh, mat }
    }
}

impl Hittable for TriangleMesh {
    fn bounding_box(&self, _time: (f64, f64)) -> Option<crate::aabb::AABB> {
        Some(self.mesh.bx)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.mesh.bx.hit(r, t_min, t_max) {
            return None;
        }
        match self.mesh.node.hit(r, t_min, t_max) {
            Some(rec) => {
                let mut new_rec = rec;
                new_rec.mat = self.mat.clone();
                Some(new_rec)
            }
            _ => None,
        }
    }
}

pub fn triangle_intersect(r: &Ray, v: (Point, Point, Point)) -> Option<(f64, (f64, f64), Vec3)> {
    let v0v1 = v.1 - v.0;
    let v0v2 = v.2 - v.0;
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

    let normal = v0v1.cross(v0v2).unit();

    // if det < 0.0 {
    //     normal = -normal;
    // }

    Some((t, (u, v), normal))
}
