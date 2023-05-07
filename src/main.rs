pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod image;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

use std::sync::Arc;

use hittable::{HitRecord, Hittable};
use ray::Ray;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::image::Image;
use crate::sphere::Sphere;
use crate::utils::random_double_normal;
use crate::vec3::{Color, Point, Vec3};

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    let mut temp_rec = HitRecord::default();
    if world.hit(r, 0.0, f64::MAX, &mut temp_rec) {
        return 0.5 * (temp_rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: u32 = 100;

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let mut image: Image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, 100);

    let camera = Camera::default();

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f64) + random_double_normal()) / (IMAGE_WIDTH - 1) as f64;
                let v = ((j as f64) + random_double_normal()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }

            image.append_color(pixel_color)
        }
    }

    image.write()
}
