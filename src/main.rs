use std::sync::{Arc, Mutex};

use rayon::ThreadPoolBuilder;
use rust_raytracer::hittable::Hittable;
use rust_raytracer::material::{Material, Scatterable};
use rust_raytracer::object::Object;
use rust_raytracer::ray::Ray;
use rust_raytracer::utils::random_double;

use rust_raytracer::camera::Camera;
use rust_raytracer::hittable_list::HittableList;
use rust_raytracer::image::Image;
use rust_raytracer::material::{Dielectric, Lambertain, Metal};
use rust_raytracer::sphere::Sphere;
use rust_raytracer::utils::random_double_normal;
use rust_raytracer::vec3::{Color, Point, Vec3};

struct RowColors {
    row: Vec<Color>,
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_mat = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.5, 0.5, 0.5,
    ))));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ))));

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
                    let mat = Arc::new(Material::Lambertain(Lambertain::new(albedo)));
                    world.add(Arc::new(Object::Sphere(Sphere::new(center, 0.2, mat))));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let mat = Arc::new(Material::Metal(Metal::new(albedo, fuzz)));
                    world.add(Arc::new(Object::Sphere(Sphere::new(center, 0.2, mat))));
                } else {
                    let mat = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
                    world.add(Arc::new(Object::Sphere(Sphere::new(center, 0.2, mat))));
                }
            }
        }
    }

    let mat1 = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        mat1,
    ))));

    let mat2 = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    ))));

    let mat3 = Arc::new(Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        mat3,
    ))));

    world
}

fn ray_color(r: &Ray, world: &Object, depth: u32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    match world.hit(&r, 0.001, f64::MAX) {
        Some(rec) => {
            let mat = rec.clone().mat;
            match mat.scatter(&r, &rec) {
                Some((Some(scattered), attenuation)) => {
                    attenuation * ray_color(&scattered, world, depth - 1)
                }
                None => Color::new_empty(),
                Some((None, _)) => todo!(),
            }
        }
        None => {
            let unit_dir = r.dir.unit();
            let t = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 600;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 50;

    let world = Object::HittableList(random_scene());
    let arc_world = Arc::new(world);

    let mut image: Image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);

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

    let mut row_colors = Vec::with_capacity(IMAGE_HEIGHT);
    row_colors.resize_with(IMAGE_HEIGHT, || RowColors { row: vec![] });

    let rows: Arc<Mutex<Vec<RowColors>>> = Arc::new(Mutex::new(row_colors));

    let pool = ThreadPoolBuilder::new().num_threads(12).build().unwrap();

    pool.scope(|s| {
        for j in 0..IMAGE_HEIGHT {
            let arc_world = arc_world.clone();
            let camera = camera.clone();
            let rows = rows.clone();
            s.spawn(move |_| {
                let mut v: Vec<Color> = Vec::new();
                for i in 0..IMAGE_WIDTH {
                    let mut pixel_color = Color::default();
                    for _s in 0..SAMPLES_PER_PIXEL {
                        let u = ((i as f64) + random_double_normal()) / (IMAGE_WIDTH - 1) as f64;
                        let v = ((j as f64) + random_double_normal()) / (IMAGE_HEIGHT - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(&r, arc_world.as_ref(), MAX_DEPTH);
                    }

                    v.push(pixel_color);
                }
                let mut data = rows.lock().unwrap();
                data[j] = RowColors { row: v };
                println!("finished {}", j);
            })
        }
    });

    for row in rows.lock().unwrap().iter().rev() {
        for color in &row.row {
            image.append_color(*color)
        }
    }

    image.write()
}
