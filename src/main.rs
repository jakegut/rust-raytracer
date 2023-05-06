pub mod image;
pub mod ray;
pub mod vec3;

use ray::Ray;

use crate::image::Image;
use crate::vec3::{Color, Point, Vec3};

fn hit_sphere(center: Point, radius: f64, ray: Ray) -> bool {
    let oc = ray.orig - center;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;
    disc > 0.0
}

fn ray_color(r: Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut image: Image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            // let color: Color = Color::new(
            //     (i as f64) / (IMAGE_WIDTH - 1) as f64,
            //     (j as f64) / (IMAGE_HEIGHT - 1) as f64,
            //     0.25,
            // );

            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r);

            image.append_color(pixel_color)
        }
    }

    image.write()
}
