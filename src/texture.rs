use std::{path::Path, sync::Arc};

use crate::{
    utils::clamp,
    vec3::{Color, Point},
};

pub enum Texture {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
    ImageTexture(ImageTexture),
}

impl TextureMat for Texture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        match self {
            Texture::SolidColor(sc) => sc.value(u, v, p),
            Texture::CheckerTexture(ct) => ct.value(u, v, p),
            Texture::ImageTexture(img) => img.value(u, v, p),
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::SolidColor(SolidColor::from_color(&Color::new(0.2, 0.2, 0.2)))
    }
}

pub trait TextureMat {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

#[derive(Default)]
pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }

    pub fn from_color(c: &Color) -> Self {
        Self::new(c.x, c.y, c.z)
    }
}

impl TextureMat for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Arc<Texture>,
    even: Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(even: Arc<Texture>, odd: Arc<Texture>) -> Self {
        CheckerTexture { odd, even }
    }

    pub fn from_colors(c0: Color, c1: Color) -> Self {
        Self::new(
            Arc::new(Texture::SolidColor(SolidColor::from_color(&c0))),
            Arc::new(Texture::SolidColor(SolidColor::from_color(&c1))),
        )
    }
}

impl TextureMat for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(path: String) -> Self {
        let p = Path::new(&path);
        // match image::open(path) {
        //     Err(e) => {
        //         println!("opening: {path}: {}", e.to_string())
        //     },
        //     Ok(d) => {
        //         let buf = d.as_rgb8();

        //     }
        // };
        let di = image::open(p).expect("Opening image");
        let data = di.to_rgb8().to_vec();
        println!();
        let width = di.width();
        let height = di.height();
        Self {
            data,
            width,
            height,
        }
    }
}

impl TextureMat for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;

        i = i.min(self.width - 1);
        j = j.min(self.height - 1);

        let color_scale = 1.0 / 255.0;
        let offset = ((j * self.width + i) * 3) as usize;

        Color::new(
            self.data[offset] as f64 * color_scale,
            self.data[offset + 1] as f64 * color_scale,
            self.data[offset + 2] as f64 * color_scale,
        )
    }
}
