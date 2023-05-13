use std::{f64::consts::PI, sync::Arc};

use crate::{
    hittable::HitRecord,
    onb::ONB,
    pdf::{CosinePDF, PDF},
    ray::Ray,
    texture::{SolidColor, Texture, TextureMat},
    utils::{random_cosine_direction, random_double_normal},
    vec3::{Color, Point, Vec3},
};

pub enum Material {
    Lambertain(Lambertain),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertain(l) => l.scatter(ray_in, hit_record),
            Material::Metal(m) => m.scatter(ray_in, hit_record),
            Material::Dielectric(d) => d.scatter(ray_in, hit_record),
            Material::DiffuseLight(dl) => dl.scatter(ray_in, hit_record),
        }
    }

    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord, u: f64, v: f64, p: &Point) -> Color {
        match self {
            Material::Lambertain(l) => l.emitted(ray_in, hit_record, u, v, p),
            Material::DiffuseLight(dl) => dl.emitted(ray_in, hit_record, u, v, p),
            Material::Metal(m) => m.emitted(ray_in, hit_record, u, v, p),
            Material::Dielectric(d) => d.emitted(ray_in, hit_record, u, v, p),
        }
    }

    fn scatter_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> Option<f64> {
        match self {
            Material::Lambertain(l) => l.scatter_pdf(ray_in, hit_record, scattered),
            Material::DiffuseLight(dl) => dl.scatter_pdf(ray_in, hit_record, scattered),
            Material::Metal(m) => m.scatter_pdf(ray_in, hit_record, scattered),
            Material::Dielectric(d) => d.scatter_pdf(ray_in, hit_record, scattered),
        }
    }
}

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
}

pub trait Scatterable {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scatter_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> Option<f64> {
        None
    }

    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord, _u: f64, _v: f64, _p: &Point) -> Color {
        Color::new_empty()
    }
}

#[derive(Default)]
pub struct Lambertain {
    albedo: Arc<Texture>,
}

impl Lambertain {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Arc::new(Texture::SolidColor(SolidColor::new(
                albedo.x, albedo.y, albedo.z,
            ))),
        }
    }

    pub fn from_texture(tx: Arc<Texture>) -> Self {
        Self { albedo: tx }
    }
}

impl Scatterable for Lambertain {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_record = ScatterRecord {
            specular_ray: None,
            attenuation: self
                .albedo
                .value(hit_record.uv.0, hit_record.uv.1, &hit_record.p),
            pdf_ptr: Some(Arc::new(CosinePDF::new(&hit_record.normal))),
        };
        Some(scatter_record)
    }

    fn scatter_pdf(&self, _ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> Option<f64> {
        let cos = hit_record.normal.dot(scattered.dir.unit());
        Some(if cos < 0.0 { 0.0 } else { cos / PI })
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
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray_in.dir.unit().reflect(hit_record.normal);
        let record = ScatterRecord {
            specular_ray: Some(Ray::new(
                hit_record.p,
                reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            )),
            attenuation: self.albedo,
            pdf_ptr: None,
        };
        Some(record)
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
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
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

        let record = ScatterRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            specular_ray: Some(Ray::new(hit_record.p, direction).with_time(ray_in.time)),
            pdf_ptr: None,
        };

        Some(record)
    }
}

pub struct DiffuseLight {
    emit: Arc<Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<Texture>) -> Self {
        Self { emit: texture }
    }

    pub fn from_color(c: &Color) -> Self {
        Self {
            emit: Arc::new(Texture::SolidColor(SolidColor::from_color(c))),
        }
    }
}

impl Scatterable for DiffuseLight {
    fn emitted(&self, _ray_in: &Ray, hit_record: &HitRecord, u: f64, v: f64, p: &Point) -> Color {
        if hit_record.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::new_empty()
        }
    }

    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}
