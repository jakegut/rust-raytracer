use core::panic;
use std::sync::Arc;

use glam::{DVec2, Vec2};

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
    pub normals: Option<(Vec3, Vec3, Vec3)>,
    pub uvs: Option<(DVec2, DVec2, DVec2)>,
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
            Some((t, mut uv, mut normal)) => {
                if t < t_max && t > t_min {
                    let mut new_rec = HitRecord::default();

                    if let Some(normals) = self.normals {
                        normal =
                            (1.0 - uv.0 - uv.1) * normals.0 + uv.0 * normals.1 + uv.1 * normals.2
                    }

                    if let Some(uvs) = self.uvs {
                        let v = (1.0 - uv.0 - uv.1) * uvs.0 + uv.0 * uvs.1 + uv.1 * uvs.2;
                        uv.0 = v.x;
                        uv.1 = v.y;
                    }

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
    pub bx: Option<AABB>,
}

impl Mesh {
    pub fn new(path: String) -> Self {
        let obj = load_obj(path).expect("invalid obj");

        let mut tris = HittableList::new();

        for face in &obj.faces {
            if face.verts.len() != 3 {
                panic!("face info doesn't have 3 verts")
            }

            let v0 = obj.vers[face.verts[0].vert_idx - 1];
            let v1 = obj.vers[face.verts[1].vert_idx - 1];
            let v2 = obj.vers[face.verts[2].vert_idx - 1];

            let mut uvs = None;
            if face.verts[0].uv_idx != 0 {
                let uv0 = obj.uvs[face.verts[0].uv_idx - 1];
                let uv1 = obj.uvs[face.verts[1].uv_idx - 1];
                let uv2 = obj.uvs[face.verts[2].uv_idx - 1];
                uvs = Some((uv0, uv1, uv2))
            }

            let mut normals = None;
            if face.verts[0].normal_idx != 0 {
                let n0 = obj.normals[face.verts[0].normal_idx - 1];
                let n1 = obj.normals[face.verts[1].normal_idx - 1];
                let n2 = obj.normals[face.verts[2].normal_idx - 1];
                normals = Some((n0, n1, n2))
            }

            tris.add(Arc::new(Object::Triangle(Triangle {
                vs: (v0, v1, v2),
                normals,
                uvs,
            })))
        }

        let node = BVHNode::new(tris, (0.0, 0.0));

        let bx = node.bounding_box((0.0, 0.0));

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
        self.mesh.bx
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

    Some((t, (u, v), normal))
}
