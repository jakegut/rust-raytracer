use std::{fs::File, io::BufWriter, path::Path};

use crate::{utils::clamp, vec3::Color};

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
    samples: u32,
}

impl Image {
    pub fn new(width: usize, height: usize, samples: u32) -> Image {
        Image {
            data: Vec::with_capacity(width * height * 3),
            width,
            height,
            samples,
        }
    }

    pub fn append_color(&mut self, color: Color) {
        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;

        let scale = 1.0 / (self.samples as f64);
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();
        self.data.push((256.0 * clamp(r, 0.0, 0.999)) as u8);
        self.data.push((256.0 * clamp(g, 0.0, 0.999)) as u8);
        self.data.push((256.0 * clamp(b, 0.0, 0.999)) as u8);
    }

    pub fn write(&mut self) {
        let path = Path::new(r"image.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(self.data.as_slice()).unwrap();
    }
}
