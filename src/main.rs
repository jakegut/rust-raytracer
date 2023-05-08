pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

use std::char::MAX;
use std::sync::Arc;

use hittable::{HitRecord, Hittable};
use ray::Ray;
use utils::random_double;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::image::Image;
use crate::material::{Dielectric, Lambertain, Metal};
use crate::sphere::Sphere;
use crate::utils::random_double_normal;
use crate::vec3::{Color, Point, Vec3};
use futures::{stream, StreamExt};

struct RowColors {
    row: Vec<Color>,
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_mat = Arc::new(Lambertain::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double_normal();
            let center = Point::new(
                (a as f64) + 0.9 * random_double_normal(),
                0.2,
                (b as f64) + 0.9 * random_double_normal(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random_normal() * Color::random_normal();
                    let mat = Arc::new(Lambertain::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Arc::new(Lambertain::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut temp_rec = HitRecord::default();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(r, 0.001, f64::MAX, &mut temp_rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let mat = temp_rec.clone().mat;
        if mat.scatter(r, temp_rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }
    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 50;

    let world = random_scene();
    let arc_world = Arc::new(world);

    let mut image: Image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, 100);

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;

    let camera = Arc::new(Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperature,
        dist_to_focus,
    ));

    let rows: Vec<RowColors> = stream::iter(0..IMAGE_HEIGHT)
        .map(move |j| {
            let arc_world = arc_world.clone();
            let camera = camera.clone();
            println!("Processing row: {}", j);
            async move {
                let mut v: Vec<Color> = Vec::new();
                for i in 0..IMAGE_WIDTH {
                    let mut pixel_color = Color::default();
                    for _s in 0..SAMPLES_PER_PIXEL {
                        let u = ((i as f64) + random_double_normal()) / (IMAGE_WIDTH - 1) as f64;
                        let v = ((j as f64) + random_double_normal()) / (IMAGE_HEIGHT - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(r, arc_world.as_ref(), MAX_DEPTH);
                    }

                    v.push(pixel_color);
                }
                RowColors { row: v }
            }
        })
        .buffered(16)
        .collect()
        .await;

    for row in rows.iter().rev() {
        for color in &row.row {
            image.append_color(*color)
        }
    }

    image.write()
}
