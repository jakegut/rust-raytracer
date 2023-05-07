pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

use std::char::MAX;
use std::sync::Arc;

use hittable::{HitRecord, Hittable};
use ray::Ray;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::image::Image;
use crate::sphere::Sphere;
use crate::utils::random_double_normal;
use crate::vec3::{Color, Point, Vec3};
use futures::{stream, StreamExt};

struct RowColors {
    idx: usize,
    row: Vec<Color>,
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut temp_rec = HitRecord::default();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(r, 0.001, f64::MAX, &mut temp_rec) {
        let target = temp_rec.p + temp_rec.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(Ray::new(temp_rec.p, target - temp_rec.p), world, depth - 1);
    }
    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
    let arc_world = Arc::new(world);

    let mut image: Image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, 100);

    let camera = Arc::new(Camera::default());

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
                RowColors { idx: j, row: v }
            }
        })
        .buffered(10)
        .collect()
        .await;

    for row in rows.iter().rev() {
        for color in &row.row {
            image.append_color(*color)
        }
    }

    image.write()
}
