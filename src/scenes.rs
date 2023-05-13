use std::sync::Arc;

use crate::{
    hittable::FlipFace,
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertain, Material, Metal},
    object::Object,
    rect::{RectBox, XYRect, XZRect, YZRect},
    sphere::{MovingSphere, Sphere},
    texture::{CheckerTexture, ImageTexture, SolidColor, Texture},
    utils::{random_double, random_double_normal},
    vec3::{Color, Point, Vec3},
};

#[derive(Default)]
pub struct SceneConfig {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aperture: f64,
    pub world: Object,
    pub lights: Arc<Object>,
    pub image_height: usize,
    pub aspect_ratio: f64,
    pub dist_to_focus: f64,
    pub background: Color,
    pub samples_per_pixel: u32,
}

pub fn new_scene(choice: u32) -> SceneConfig {
    let (mut cfg, world): (SceneConfig, HittableList) = match choice {
        1 => (
            SceneConfig {
                lookfrom: Vec3::new(13.0, 2.0, 3.0),
                lookat: Point::new(0.0, 0.0, 0.0),
                vfov: 20.0,
                aperture: 0.1,
                background: Color::new(0.7, 0.8, 1.0),
                ..Default::default()
            },
            random_scene(),
        ),
        4 => (
            SceneConfig {
                lookfrom: Vec3::new(13.0, 2.0, 3.0),
                lookat: Point::new(0.0, 0.0, 0.0),
                background: Color::new(0.7, 0.8, 1.0),
                vfov: 20.0,
                ..Default::default()
            },
            earth(),
        ),
        2 => (
            SceneConfig {
                lookfrom: Vec3::new(13.0, 2.0, 3.0),
                lookat: Point::new(0.0, 0.0, 0.0),
                vfov: 20.0,
                ..Default::default()
            },
            two_spheres(),
        ),
        5 => (
            SceneConfig {
                lookfrom: Point::new(26.0, 3.0, 6.0),
                lookat: Point::new(0.0, 2.0, 0.0),
                vfov: 20.0,
                background: Color::new(0.0, 0.0, 0.0),
                samples_per_pixel: 400,
                ..Default::default()
            },
            simple_light(),
        ),
        6 | _ => (
            SceneConfig {
                aspect_ratio: 1.0,
                samples_per_pixel: 200,
                background: Color::new_empty(),
                lookfrom: Vec3::new(278.0, 278.0, -800.0),
                lookat: Vec3::new(278.0, 278.0, 0.0),
                vfov: 40.0,
                ..Default::default()
            },
            cornell_box(),
        ),
    };

    let lights: Vec<Arc<Object>> = world
        .objects
        .iter()
        .filter(|&o| o.is_light())
        .cloned()
        .collect();
    let lights_hittable = Object::HittableList(HittableList { objects: lights });
    cfg.world = Object::HittableList(world);
    cfg.lights = lights_hittable.into();
    cfg.samples_per_pixel = cfg.samples_per_pixel.max(100);
    cfg.vup = Vec3::new(0.0, 1.0, 0.0);
    cfg.dist_to_focus = 10.0;
    cfg.aspect_ratio = if cfg.aspect_ratio == 0.0 {
        16.0 / 9.0
    } else {
        cfg.aspect_ratio
    };
    return cfg;
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.65, 0.05, 0.05,
    ))));
    let white = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.73, 0.73, 0.73,
    ))));
    let green = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.12, 0.45, 0.15,
    ))));
    let light = Arc::new(Material::DiffuseLight(DiffuseLight::from_color(
        &Color::new(20.0, 20.0, 20.0),
    )));

    world.add(Arc::new(Object::YZRect(YZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        green,
    ))));
    world.add(Arc::new(Object::YZRect(YZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        red,
    ))));
    world.add(Arc::new(Object::XZRect(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    ))));
    world.add(Arc::new(Object::XZRect(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ))));
    world.add(Arc::new(Object::XYRect(XYRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ))));

    // world.add(Arc::new(Object::RectBox(RectBox::new(
    //     &Point::new(130.0, 0.0, 65.0),
    //     &Point::new(295.0, 165.0, 230.0),
    //     white.clone(),
    // ))));

    // let aluminum = Arc::new(Material::Metal(Metal::new(
    //     Color::new(0.8, 0.85, 0.88),
    //     0.0,
    // )));
    world.add(Arc::new(Object::RectBox(RectBox::new(
        &Point::new(265.0, 0.0, 295.0),
        &Point::new(430.0, 330.0, 460.0),
        white.clone(),
    ))));

    let glass = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(190.0, 190.0, 190.0),
        90.0,
        glass,
    ))));

    let light_obj = Arc::new(Object::XZRect(XZRect::new(
        (213.0, 343.0),
        (277.0, 332.0),
        554.0,
        light,
    )));

    world.add(Arc::new(Object::FlipFace(FlipFace::new(light_obj))));

    world
}

fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let odd = Color::new(1.0, 0.0, 1.0);
    let even = Color::new(0.2, 0.2, 0.2);
    let checker_texture = Arc::new(Texture::CheckerTexture(CheckerTexture::from_colors(
        odd, even,
    )));
    let checker_mat = Arc::new(Material::Lambertain(Lambertain::from_texture(
        checker_texture,
    )));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        checker_mat.clone(),
    ))));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        checker_mat.clone(),
    ))));

    let difflight = Arc::new(Material::DiffuseLight(DiffuseLight::from_color(
        &Color::new(4.0, 4.0, 4.0),
    )));
    let xyrect = Object::XYRect(XYRect::new((3.0, 5.0), (1.0, 3.0), -2.0, difflight));
    world.add(Arc::new(xyrect));

    world
}

fn earth() -> HittableList {
    let earth_texture = Arc::new(Texture::ImageTexture(ImageTexture::new(
        "earthmap.png".to_string(),
    )));
    let earth_surface = Arc::new(Material::Lambertain(Lambertain::from_texture(
        earth_texture,
    )));
    let globe = Arc::new(Object::Sphere(Sphere::new(
        Vec3::new_empty(),
        2.0,
        earth_surface,
    )));

    let mut world = HittableList::new();
    world.add(globe);
    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(Texture::CheckerTexture(CheckerTexture::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));
    let mat = Arc::new(Material::Lambertain(Lambertain::from_texture(checker)));

    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        mat.clone(),
    ))));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        mat.clone(),
    ))));

    world
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(Texture::CheckerTexture(CheckerTexture::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));
    let ground_mat = Arc::new(Material::Lambertain(Lambertain::from_texture(checker)));
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
                    let albedo = Color::new(
                        (center.x + 11.0) / 22.0,
                        center.y / 22.0,
                        (center.z + 11.0) / 22.0,
                    );
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
