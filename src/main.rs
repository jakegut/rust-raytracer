#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
use std::thread;

use eframe::egui;
use egui::{Color32, ColorImage, Vec2};
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
use rust_raytracer::sphere::{MovingSphere, Sphere};
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
                    let center1 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.add(Arc::new(Object::MovingSphere(MovingSphere::new(
                        (center, center1),
                        (0.0, 1.0),
                        0.2,
                        mat,
                    ))));
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

fn raytrace(image_width: usize, aspect_ratio: f64, frame: Arc<RwLock<ColorImage>>) {
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let world = Object::HittableList(random_scene());
    let arc_world = Arc::new(world);

    let image: Image = Image::new(image_width, image_height, SAMPLES_PER_PIXEL, frame);
    let arc_image = Arc::new(Mutex::new(image));

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;

    let camera = Arc::new(
        Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            aspect_ratio,
            aperature,
            dist_to_focus,
        )
        .with_time(0.0, 1.0),
    );

    // let mut row_colors = Vec::with_capacity(image_height);
    // row_colors.resize_with(image_height, || RowColors { row: vec![] });

    // let rows: Arc<Mutex<Vec<RowColors>>> = Arc::new(Mutex::new(row_colors));

    let pool = ThreadPoolBuilder::new().num_threads(12).build().unwrap();

    pool.scope(|s| {
        for j in 0..image_height {
            let arc_world = arc_world.clone();
            let camera = camera.clone();
            // let rows = rows.clone();
            let image = arc_image.clone();
            s.spawn(move |_| {
                let mut colors = vec![Color::new_empty(); image_width];
                for i in 0..image_width {
                    let mut pixel_color = Color::default();
                    for _s in 0..SAMPLES_PER_PIXEL {
                        let u = ((i as f64) + random_double_normal()) / (image_width - 1) as f64;
                        let v = ((j as f64) + random_double_normal()) / (image_height - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(&r, arc_world.as_ref(), MAX_DEPTH);
                    }
                    colors[i] = pixel_color;
                }
                let mut img = image.lock().unwrap();
                for (i, color) in colors.iter().enumerate() {
                    img.append_color(color, i, image_height - j - 1);
                }
                // let mut data = rows.lock().unwrap();
                // data[j] = RowColors { row: v };
                // println!("finished {}", j);
            })
        }
    });

    // for row in rows.lock().unwrap().iter().rev() {
    //     for color in &row.row {
    //         image.append_color(*color)
    //     }
    // }

    // image.write()
}

fn main() {
    let width: usize = 600;
    let aspect_ratio: f64 = 16.0 / 9.0;
    let height = (width as f64 / aspect_ratio) as usize;
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(width as f32, height as f32)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui app",
        native_options,
        Box::new(move |cc| Box::new(MyEguiApp::new(cc, width, aspect_ratio))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    texture: Option<egui::TextureHandle>,
    frame_thing: Arc<RwLock<ColorImage>>,
    current_frame: Option<ColorImage>,
    image_width: usize,
    aspect_ratio: f64,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, image_width: usize, aspect_ratio: f64) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let height = (image_width as f64 / aspect_ratio) as usize;
        let frame_thing = Arc::new(RwLock::new(ColorImage::new(
            [image_width, height],
            Color32::BLACK,
        )));
        {
            let frame_thing = frame_thing.clone();
            thread::spawn(move || raytrace(image_width, aspect_ratio, frame_thing));
        }
        Self {
            frame_thing: frame_thing.clone(),
            image_width,
            aspect_ratio,
            ..Default::default()
        }
    }
}

fn image_as_u8(image: &ColorImage) -> Vec<u8> {
    image
        .pixels
        .iter()
        .flat_map(Color32::to_array)
        .collect()
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let frame_thing = self.frame_thing.read().unwrap();
            let current_image = frame_thing.clone();
            self.texture = Some(
                ui.ctx()
                    .load_texture("scene", current_image.clone(), Default::default()),
            );

            if ui.button("save image").clicked() {
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    let res = image::save_buffer_with_format(
                        path.display().to_string(),
                        &image_as_u8(&current_image),
                        current_image.width() as u32,
                        current_image.height() as u32,
                        image::ColorType::Rgba8,
                        image::ImageFormat::Png,
                    );

                    match res {
                        Err(e) => println!("error: {}", e.to_string()),
                        Ok(_) => {}
                    }
                };
            };

            if let Some(texture) = self.texture.as_ref() {
                ui.image(texture, ui.available_size())
            } else {
                ui.spinner()
            };

            ctx.request_repaint()
        });
    }
}
