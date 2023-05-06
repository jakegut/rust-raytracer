pub mod image;
pub mod vec3;
use crate::image::Image;
use crate::vec3::Color;

fn main() {
    const IMAGE_HEIGHT: usize = 256;
    const IMAGE_WIDTH: usize = 256;

    let mut image: Image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let color: Color = Color::new(
                (i as f64) / (IMAGE_WIDTH - 1) as f64,
                (j as f64) / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            image.append_color(color)
        }
    }

    image.write()
}
