pub mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png;

fn main() {
    const IMAGE_HEIGHT: u32 = 256;
    const IMAGE_WIDTH: u32 = 256;

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    const DATA_SIZE: usize = (IMAGE_HEIGHT * IMAGE_WIDTH * 3) as usize;
    let mut data: [u8; DATA_SIZE] = [0; DATA_SIZE];
    let mut data_i = 0;

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r: f32 = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            data[data_i] = ir;
            data[data_i + 1] = ig;
            data[data_i + 2] = ib;

            data_i += 3;
        }
    }

    writer.write_image_data(&data).unwrap();
}
