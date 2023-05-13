use crate::vec3::Vec3;

#[derive(Default)]
pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn new() -> Self {
        ONB::default()
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u * a + self.v * b + self.w * c
    }

    pub fn local_vec(&self, v: &Vec3) -> Vec3 {
        self.local(v.x, v.y, v.z)
    }

    pub fn from_w(n: &Vec3) -> Self {
        let w = n.unit();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).unit();
        let u = w.cross(v);
        Self { u, v, w }
    }
}
