use std::{
    fs::File,
    io::BufWriter,
    path::Path,
    sync::{Arc, RwLock},
};

use eframe::epaint::{Color32, ColorImage};

use crate::{utils::clamp, vec3::Color};

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
    samples: u32,
    frame: Arc<RwLock<ColorImage>>,
}

impl Image {
    pub fn new(width: usize, height: usize, samples: u32, frame: Arc<RwLock<ColorImage>>) -> Image {
        Image {
            data: Vec::with_capacity(width * height * 3),
            width,
            height,
            samples,
            frame,
        }
    }

    pub fn append_color(&mut self, color: &Color, x: usize, y: usize) {
        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;

        let scale = 1.0 / (self.samples as f64);
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        let color = Color32::from_rgb(
            (256.0 * clamp(r, 0.0, 0.999)) as u8,
            (256.0 * clamp(g, 0.0, 0.999)) as u8,
            (256.0 * clamp(b, 0.0, 0.999)) as u8,
        );

        let mut frame = self.frame.write().unwrap();
        frame[(x, y)] = color;

        // self.data.push(color.r());
        // self.data.push(color.b());
        // self.data.push(color.g());
    }

    // pub fn write(&mut self) {
    //     let path = Path::new(r"image.png");
    //     let file = File::create(path).unwrap();
    //     let ref mut w = BufWriter::new(file);

    //     let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
    //     encoder.set_color(png::ColorType::Rgb);
    //     encoder.set_depth(png::BitDepth::Eight);

    //     let mut writer = encoder.write_header().unwrap();

    //     writer.write_image_data(self.data.as_slice()).unwrap();
    // }
}
