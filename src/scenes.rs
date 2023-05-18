use std::{f64::consts::PI, sync::Arc};

use crate::{
    hittable::{FlipFace, MatTransform},
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertain, Material, Metal},
    mesh::{Mesh, TriangleMesh},
    object::Object,
    rect::{RectBox, XYRect, XZRect, YZRect},
    sphere::{MovingSphere, Sphere},
    texture::{CheckerTexture, ImageTexture, Texture},
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
        6 => (
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
        7 => (
            SceneConfig {
                aspect_ratio: 1.0,
                samples_per_pixel: 200,
                background: Color::new(0.5, 0.6, 0.4),
                lookat: Vec3::new(15.0, 0.0, 10.0),
                lookfrom: Vec3::new(15.00001, 60.0, 10.001),
                vfov: 40.0,
                ..Default::default()
            },
            teapot_galore(),
        ),
        8 | _ => (
            SceneConfig {
                aspect_ratio: 1.0,
                samples_per_pixel: 200,
                background: Color::new_empty(),
                lookfrom: Vec3::new(278.0, 278.0, -800.0),
                lookat: Vec3::new(278.0, 278.0, 0.0),
                vfov: 40.0,
                ..Default::default()
            },
            dragon_cornell(),
        ),
    };

    let lights = world.get_lights();
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

fn teapot_galore() -> HittableList {
    let mut world = HittableList::new();

    let white = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.73, 0.73, 0.73,
    ))));

    let mesh = Arc::new(Mesh::new("data/teapot.obj".into()));

    for j in 0..5 {
        for i in 0..5 {
            let trans = glam::DVec3::new((i as f64) * 6.0, 1.0, (j as f64) * 6.0);

            let rot = glam::DQuat::from_axis_angle(
                Vec3::random_unit_vector().into(),
                random_double(0.0, 2.0 * PI),
            );

            let teapot = Arc::new(Object::TriangleMesh(TriangleMesh::new(mesh.clone(), white.clone())));

            let mesh_mat = glam::DMat4::from_scale_rotation_translation(
                glam::DVec3::ONE * 1.0,
                rot,
                trans,
            );

            world.add(Arc::new(Object::MatTransform(MatTransform::new(
                mesh_mat,
                teapot.clone(),
            ))));
        }
    }

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

    let diff_light = Arc::new(Material::DiffuseLight(DiffuseLight::from_color(
        &Color::new(1.0, 1.0, 1.0),
    )));
    world.add(Arc::new(Object::Sphere(Sphere::new(
        Point::new(0.0, 50.0, 0.0),
        1.0,
        diff_light,
    ))));
    world
}

fn dragon_cornell() -> HittableList {
    let mut world = empty_cornell();

    let white = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.73, 0.73, 0.73,
    ))));

    let glass = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
    let aluminum = Arc::new(Material::Metal(Metal::new(
        Color::new(0.8, 0.85, 0.88),
        0.4,
    )));

    let big_mat = glam::DMat4::from_scale_rotation_translation(
        glam::DVec3::ONE * 55.0,
        glam::DQuat::from_rotation_y(195_f64.to_radians()),
        glam::DVec3 {
            x: 275.0,
            y: 0.0,
            z: 400.0,
        },
    );

    let glass_mat = glam::DMat4::from_scale_rotation_translation(
        glam::DVec3::ONE * 30.0,
        glam::DQuat::from_rotation_y(150_f64.to_radians()),
        glam::DVec3 {
            x: 150.0,
            y: 0.0,
            z: 200.0,
        },
    );

    let alum_mat = glam::DMat4::from_scale_rotation_translation(
        glam::DVec3::ONE * 30.0,
        glam::DQuat::from_rotation_y(210_f64.to_radians()),
        glam::DVec3 {
            x: 400.0,
            y: 0.0,
            z: 200.0,
        },
    );

    // let big_mat = glam::DMat4::IDENTITY;

    let mesh = Arc::new(Mesh::new("data/dragon.obj".into()));

    let big_dragon = Object::TriangleMesh(TriangleMesh::new(mesh.clone(), white.clone()));
    let glass_dragon = Object::TriangleMesh(TriangleMesh::new(mesh.clone(), glass.clone()));
    let alum_dragon = Object::TriangleMesh(TriangleMesh::new(mesh.clone(), aluminum.clone()));

    let big_scale = Object::MatTransform(MatTransform::new(big_mat, Arc::new(big_dragon)));
    let glass_scale = Object::MatTransform(MatTransform::new(glass_mat, Arc::new(glass_dragon)));
    let alum_scale = Object::MatTransform(MatTransform::new(alum_mat, Arc::new(alum_dragon)));

    world.add(Arc::new(big_scale));
    world.add(Arc::new(glass_scale));
    world.add(Arc::new(alum_scale));

    world
}

fn empty_cornell() -> HittableList {
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
        &(Color::new(20.0, 15.0, 10.0) * 5.0),
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

    let light_obj = Arc::new(Object::XZRect(XZRect::new(
        (213.0, 343.0),
        (277.0, 332.0),
        554.0,
        light,
    )));

    world.add(Arc::new(Object::FlipFace(FlipFace::new(light_obj))));

    world
}

fn cornell_box() -> HittableList {
    let mut world = empty_cornell();

    let white = Arc::new(Material::Lambertain(Lambertain::new(Color::new(
        0.73, 0.73, 0.73,
    ))));

    let box0 = Arc::new(Object::RectBox(RectBox::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 165.0, 165.0),
        white.clone(),
    )));
    let box0_mat = glam::DMat4::from_rotation_translation(
        glam::DQuat::from_rotation_y(-20.0_f64.to_radians()),
        glam::DVec3 {
            x: 125.0,
            y: 0.0,
            z: 100.0,
        },
    );
    world.add(Arc::new(Object::MatTransform(MatTransform::new(
        box0_mat, box0,
    ))));

    // let aluminum = Arc::new(Material::Metal(Metal::new(
    //     Color::new(0.8, 0.85, 0.88),
    //     0.0,
    // )));

    let box1 = Arc::new(Object::RectBox(RectBox::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    let box1_mat = glam::DMat4::from_rotation_translation(
        glam::DQuat::from_rotation_y(15.0_f64.to_radians()),
        glam::DVec3 {
            x: 250.0,
            y: 0.0,
            z: 300.0,
        },
    );
    let box1_mat_transform = Object::MatTransform(MatTransform::new(box1_mat, box1));
    world.add(Arc::new(box1_mat_transform));

    // world.add(sphere_obj);

    // let light_mat = glam::DMat4::from_rotation_translation(
    //     glam::DQuat::from_rotation_x(PI),
    //     glam::DVec3 {
    //         x: 200.0,
    //         y: 150.0,
    //         z: 100.0,
    //     },
    // );

    // let light_mat = glam::DMat4::from_translation(glam::DVec3 {
    //     x: 0.0,
    //     y: 50.0,
    //     z: 0.0,
    // });

    let mesh_mat = glam::DMat4::from_scale_rotation_translation(
        glam::DVec3::ONE * 25.0,
        glam::DQuat::from_rotation_y(105_f64.to_radians()),
        glam::DVec3 {
            x: 300.0,
            y: 50.0,
            z: 100.0,
        },
    );

    let mesh = Arc::new(Mesh::new("data/teapot.obj".into()));

    let teapot = Object::TriangleMesh(TriangleMesh::new(mesh, white.clone()));
    let mesh_scale = Object::MatTransform(MatTransform::new(mesh_mat, Arc::new(teapot)));
    world.add(Arc::new(mesh_scale));

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
