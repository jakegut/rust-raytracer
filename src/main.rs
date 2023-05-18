#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

use eframe::{egui, Error};
use egui::{Color32, ColorImage, Vec2};
use rayon::ThreadPoolBuilder;

use rust_raytracer::hittable::Hittable;
use rust_raytracer::material::Scatterable;
use rust_raytracer::object::Object;
use rust_raytracer::pdf::{HittablePDF, MixturePDF, PDF};
use rust_raytracer::ray::Ray;
use rust_raytracer::scenes::{new_scene, SceneConfig};
use rust_raytracer::BIAS;

use rust_raytracer::camera::Camera;

use rust_raytracer::image::Image;

use rust_raytracer::utils::random_double_normal;
use rust_raytracer::vec3::Color;

fn ray_color(
    r: &Ray,
    world: &Object,
    lights: Arc<Object>,
    background: &Color,
    depth: u32,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    match world.hit(&r, 0.001, f64::MAX) {
        Some(rec) => {
            let emitted = rec.mat.emitted(&r, &rec, rec.uv.0, rec.uv.1, &rec.p);
            let mat = rec.clone().mat;
            match mat.scatter(&r, &rec) {
                Some(srec) => {
                    let light_pdf = Arc::new(HittablePDF::new(rec.p, lights.clone()));
                    // let scattered = Ray::new(rec.p, light_pdf.generate()).with_time(r.time);
                    // let pdf_val = light_pdf.value(&scattered.dir);

                    if let Some(specular_ray) = srec.specular_ray {
                        return srec.attenuation
                            * ray_color(
                                &specular_ray,
                                world,
                                lights.clone(),
                                background,
                                depth - 1,
                            );
                    }

                    let pdf: MixturePDF = match srec.pdf_ptr {
                        Some(pdf_ptr) => MixturePDF::new(light_pdf, pdf_ptr.clone()),
                        None => MixturePDF::new(light_pdf.clone(), light_pdf.clone()),
                    };

                    let scattered =
                        Ray::new(rec.p + rec.normal * BIAS, pdf.generate()).with_time(r.time);
                    let pdf_val = pdf.value(&scattered.dir);

                    let scat_pdf = match rec.mat.scatter_pdf(&r, &rec, &scattered) {
                        Some(v) => v,
                        None => 0.0,
                    };

                    emitted
                        + srec.attenuation
                            * scat_pdf
                            * ray_color(&scattered, world, lights.clone(), background, depth - 1)
                            / pdf_val
                    // srec.attenuation + light_color + ray_color(&scattered, world, lights.clone(), background, depth - 1)
                }
                None => emitted,
            }
        }
        None => *background,
    }
}

fn raytrace(image_width: usize, scene_config: Arc<SceneConfig>, frame: Arc<RwLock<ColorImage>>) {
    let image_height: usize = (image_width as f64 / scene_config.aspect_ratio) as usize;
    let samples_per_pixel: u32 = 100;
    const MAX_DEPTH: u32 = 5;

    let world = &scene_config.world;
    let arc_world = Arc::new(world);

    let image: Image = Image::new(image_width, image_height, samples_per_pixel, frame);
    let arc_image = Arc::new(Mutex::new(image));

    let lights = scene_config.lights.clone();

    let camera = Arc::new(
        Camera::new(
            scene_config.lookfrom,
            scene_config.lookat,
            scene_config.vup,
            scene_config.vfov,
            scene_config.aspect_ratio,
            scene_config.aperture,
            scene_config.dist_to_focus,
        )
        .with_time(0.0, 1.0),
    );

    let pool = ThreadPoolBuilder::new().num_threads(12).build().unwrap();

    let mut chunks: Vec<(usize, usize)> = vec![];
    let chunk_size = 16;
    let chunk_width = image_height / chunk_size;
    let chunk_height = image_height / chunk_size;

    for j in 0..chunk_height {
        for i in 0..chunk_width {
            chunks.push((i, j))
        }
    }

    let start = Instant::now();
    let background = scene_config.background;
    pool.scope(|s| {
        for chunk in chunks {
            let arc_world = arc_world.clone();
            let camera = camera.clone();
            let image = arc_image.clone();
            let arc_lights = lights.clone();
            s.spawn(move |_| {
                let mut colors =
                    vec![Color::new(samples_per_pixel as f64, 0.0, 0.0); chunk_size * chunk_size];
                let mut img = image.lock().unwrap();
                for (i, color) in colors.iter().enumerate() {
                    let pos_y = chunk_size * chunk.0 + (i % chunk_size);
                    let pos_x = chunk_size * chunk.1 + (i / chunk_size);

                    if pos_y > image_height || pos_x > image_width {
                        continue;
                    }

                    img.append_color(color, pos_x, image_height - pos_y - 1);
                }
                drop(img);
                for j in 0..chunk_size {
                    let y = chunk_size * chunk.1 + j;
                    if y > image_height {
                        continue;
                    }
                    for i in 0..chunk_size {
                        let x = chunk_size * chunk.0 + i;
                        if x > image_width {
                            continue;
                        }
                        let mut pixel_color = Color::default();
                        for _s in 0..samples_per_pixel {
                            let u =
                                ((y as f64) + random_double_normal()) / (image_width - 1) as f64;
                            let v =
                                ((x as f64) + random_double_normal()) / (image_height - 1) as f64;
                            let r = camera.get_ray(u, v);
                            pixel_color += ray_color(
                                &r,
                                &arc_world,
                                arc_lights.clone(),
                                &background,
                                MAX_DEPTH,
                            );
                        }
                        colors[j * chunk_size + i] = pixel_color;
                    }
                }
                let mut img = image.lock().unwrap();
                for (i, color) in colors.iter().enumerate() {
                    let pos_y = chunk_size * chunk.0 + (i % chunk_size);
                    let pos_x = chunk_size * chunk.1 + (i / chunk_size);

                    if pos_y > image_height || pos_x > image_width {
                        continue;
                    }

                    img.append_color(color, pos_x, image_height - pos_y - 1);
                }
            })
        }
    });
    let duration = start.elapsed();
    println!("Time elapsed in ray_trace() is: {:?}", duration);
}

fn main() -> Result<(), Error> {
    let width: usize = 700;
    let scene_cfg = new_scene(7);
    let height = (width as f64 / scene_cfg.aspect_ratio) as usize;
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(width as f32, height as f32)),
        renderer: eframe::Renderer::Glow,
        resizable: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui app",
        native_options,
        Box::new(move |cc| Box::new(MyEguiApp::new(cc, width, Arc::new(scene_cfg)))),
    )
}

#[derive(Default)]
struct MyEguiApp {
    texture: Option<egui::TextureHandle>,
    frame_thing: Arc<RwLock<ColorImage>>,
}

impl MyEguiApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        image_width: usize,
        scene_config: Arc<SceneConfig>,
    ) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let height = (image_width as f64 / scene_config.aspect_ratio) as usize;
        let frame_thing = Arc::new(RwLock::new(ColorImage::new(
            [image_width, height],
            Color32::BLACK,
        )));
        {
            let frame_thing = frame_thing.clone();
            let scene_config = scene_config.clone();
            thread::spawn(move || {
                raytrace(image_width, scene_config, frame_thing);
            });
        }
        Self {
            frame_thing: frame_thing.clone(),
            ..Default::default()
        }
    }
}

fn image_as_u8(image: &ColorImage) -> Vec<u8> {
    image.pixels.iter().flat_map(Color32::to_array).collect()
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let frame_thing = self.frame_thing.read().unwrap();
            let current_image = frame_thing.clone();
            self.texture = Some(ui.ctx().load_texture(
                "scene",
                current_image.clone(),
                Default::default(),
            ));

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
