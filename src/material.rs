use crate::color::Color;
use crate::texture::Texture;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub texture: Option<Rc<Texture>>,
    pub emmisive_color: Color,
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture: Option<Rc<Texture>>, 
        emmisive_color: Color,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            texture,
            emmisive_color,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 0.0,
            texture: None,
            emmisive_color: Color::new(0, 0, 0),
        }
    }

    pub fn get_diffuse(&self, u: f32, v: f32) -> Color {
    if let Some(texture) = &self.texture {
        let x = ((u * (texture.width as f32 - 1.0)) as u32).min(texture.width - 1);
        let y = ((v * (texture.height as f32 - 1.0)) as u32).min(texture.height - 1);
        texture.get_pixel_color(x, y)
    } else {
        self.diffuse
    }
}

}

