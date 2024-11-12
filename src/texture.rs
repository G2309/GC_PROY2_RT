use image::{DynamicImage, GenericImageView, ImageReader};
use crate::color::Color;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Texture {
    pub image: Rc<DynamicImage>,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(file_path: &str) -> Self {
        let img = ImageReader::open(file_path)
            .expect("No se ha podido abrir la imagen")
            .decode()
            .expect("No se ha podido decodificar la imagen");
        let width = img.width();
        let height = img.height();
        let image = Rc::new(img);
        Texture { image, width, height }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> Color {
        if x >= self.width || y >= self.height {
            return Color::new(255, 0, 0); // Color de error
        }
        let pixel = self.image.get_pixel(x, y);
        Color::new(pixel[0], pixel[1], pixel[2])
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let x = (u * self.width as f32).clamp(0.0, self.width as f32 - 1.0) as u32;
        let y = (v * self.height as f32).clamp(0.0, self.height as f32 - 1.0) as u32;
        self.get_pixel_color(x, y)
    }

    pub fn sample(&self, uv: (f32, f32)) -> Color {
        let (u, v) = uv;
        self.get_color(u, v)
    }
}

