use nalgebra_glm::Vec3;
use std::rc::Rc;
use crate::cube::Cube;
use crate::material::Material;
use crate::texture::Texture;
use crate::color::Color; 
use crate::ray_intersect::RayIntersect;
use crate::square::Square; 

pub fn create_diorama() -> Vec<Box<dyn RayIntersect>> {
    let ice_texture = Rc::new(Texture::new("src/textures/blue_ice.png"));

    // Material
    let ice = Rc::new(Material::new(
        Color::new(30, 30, 30), 
        80.0, 
        [0.6, 0.4, 0.3, 0.0], 
        0.0, 
        Some(ice_texture), 
        Color::new(0,0,0),
    ));


       let cube_data = [

           //suelo
           (Vec3::new(-1.0, 0.8, -3.4), Rc::clone(&ice)),
           (Vec3::new(-1.4, 0.8, -3.4), Rc::clone(&ice)),
           (Vec3::new(-1.8, 0.8, -3.4), Rc::clone(&ice)),
           (Vec3::new(-2.2, 0.8, -3.4), Rc::clone(&ice)),
           (Vec3::new(-2.6, 0.8, -3.4), Rc::clone(&ice)),
           (Vec3::new(-3.0, 0.8, -3.4), Rc::clone(&ice)),
    ];

    let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    for &(center, ref material) in &cube_data {
        let cube = Box::new(Cube {
            center,
            size: 0.4,
            material: Rc::clone(material),
        }) as Box<dyn RayIntersect>;
        objects.push(cube);
    }

    let ground = Box::new(Square {
        center: Vec3::new(-2.0, 0.7, -4.0), 
        size: 10.0, 
        material: Rc::clone(&ice),
    }) as Box<dyn RayIntersect>;

    objects.push(ground);
    

    objects
}
