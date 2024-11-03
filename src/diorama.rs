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
    let snow = Rc::new(Texture::new("src/textures/snow.png"));

    // Material
    let ice = Rc::new(Material::new(
        Color::new(37, 150, 190), 
        80.0, 
        [0.5, 0.6, 0.5, 0.0], 
        1.3, 
        Some(ice_texture), 
        Color::new(5,10,50),
    ));
    let snow = Rc::new(Material::new(
        Color::new(255, 255, 255), 
        10.0, 
        [0.8, 0.1, 0.1, 0.0], 
        0.0, 
        Some(snow), 
        Color::new(0,0,0),
    ));


       let cube_data = [

           // Arch - r0
           (Vec3::new(4.6, 0.9, -5.0), Rc::clone(&ice)),
           (Vec3::new(-8.6, 0.9, -5.0), Rc::clone(&ice)),
           (Vec3::new(4.2, 0.9, -5.0), Rc::clone(&ice)),
           (Vec3::new(-8.2, 0.9, -5.0), Rc::clone(&ice)),
           // Arch - r1
           (Vec3::new(4.2, 1.3, -5.0), Rc::clone(&ice)),
           (Vec3::new(-8.2, 1.3, -5.0), Rc::clone(&ice)),
           (Vec3::new(3.8, 1.3, -5.0), Rc::clone(&ice)),
           (Vec3::new(-7.8, 1.3, -5.0), Rc::clone(&ice)),
           (Vec3::new(3.4, 1.3, -5.0), Rc::clone(&ice)),
           (Vec3::new(-7.4, 1.3, -5.0), Rc::clone(&ice)),
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
        size: 13.6, 
        material: Rc::clone(&snow),
    }) as Box<dyn RayIntersect>;

    objects.push(ground);
    

    objects
}
