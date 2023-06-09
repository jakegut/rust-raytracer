use crate::{
    ray::Ray,
    utils::random_double,
    vec3::{Point, Vec3},
};

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    time: (f64, f64),
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            time: (0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
            time: random_double(self.time.0, self.time.1),
        }
    }

    pub fn with_time(&mut self, t0: f64, t1: f64) -> Self {
        self.time = (t0, t1);
        *self
    }
}
