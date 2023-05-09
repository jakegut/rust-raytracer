use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::random_double_normal,
    vec3::{Color, Vec3},
};

pub enum Material {
    Lambertain(Lambertain),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        match self {
            Material::Lambertain(l) => l.scatter(ray_in, hit_record),
            Material::Metal(m) => m.scatter(ray_in, hit_record),
            Material::Dielectric(d) => d.scatter(ray_in, hit_record),
        }
    }
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)>;
}

#[derive(Default)]
pub struct Lambertain {
    albedo: Color,
}

impl Lambertain {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertain {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
        let mut scatter_dir = hit_record.normal + Vec3::random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal;
        }
        Some((Some(Ray::new(hit_record.p, scatter_dir)), self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
        let reflected = ray_in.dir.unit().reflect(hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.dir.dot(hit_record.normal) > 0.0 {
            Some((Some(scattered), self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflactance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_vec = ray_in.dir.unit();
        let cos_theta = (-unit_vec).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction: Vec3 = if cannot_refract
            || Dielectric::reflactance(cos_theta, refraction_ratio) > random_double_normal()
        {
            unit_vec.reflect(hit_record.normal)
        } else {
            unit_vec.refract(hit_record.normal, refraction_ratio)
        };

        Some((
            Some(Ray::new(hit_record.p, direction)),
            Color::new(1.0, 1.0, 1.0),
        ))
    }
}
