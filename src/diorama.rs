use nalgebra_glm::Vec3;
use crate::cube::Cube;
use crate::material::Material;
use crate::texture::Texture;
use crate::color::Color; 
use crate::ray_intersect::RayIntersect;
use crate::square::Square; 
use std::sync::Arc;

pub fn create_diorama() -> Vec<Arc<dyn RayIntersect + Send + Sync>> {
    let ice_texture = Arc::new(Texture::new("src/textures/blue_ice.png"));
    let snow = Arc::new(Texture::new("src/textures/snow.png"));

    // Material
    let ice = Arc::new(Material::new(
        Color::new(37, 150, 190), 
        80.0, 
        [0.5, 0.6, 0.5, 0.0], 
        1.3, 
        Some(ice_texture), 
        Color::new(5,10,75),
    ));
    let snow = Arc::new(Material::new(
        Color::new(255, 255, 255), 
        5.0, 
        [1.0, 0.1, 0.1, 0.0], 
        0.0, 
        Some(snow), 
        Color::new(0,0,0),
    ));


       let cube_data = [

           // Arch - r0
           (Vec3::new(2.6, 0.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-6.6, 0.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(2.2, 0.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-6.2, 0.9, -5.0), Arc::clone(&ice)),
           // Arch - r1
           (Vec3::new(2.6, 1.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-6.6, 1.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(2.2, 1.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-6.2, 1.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.8, 1.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.8, 1.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.4, 1.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.4, 1.3, -5.0), Arc::clone(&ice)),
           //Arch - r2
           (Vec3::new(2.2, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-6.2, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.8, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.8, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.8, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.8, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.4, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.4, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.0, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.0, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 1.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 1.7, -5.0), Arc::clone(&ice)),
           // Arch - r3
           (Vec3::new(-6.2, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.8, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.4, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.4, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.0, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.0, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 2.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 2.1, -5.0), Arc::clone(&ice)),
           // Arch - r4
           (Vec3::new(-5.8, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.4, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.0, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 2.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 2.5, -5.0), Arc::clone(&ice)),
           //Arch - r5
           (Vec3::new(-5.8, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.4, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.0, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.8, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.4, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 2.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 2.9, -5.0), Arc::clone(&ice)),
           //Arch - r6
           (Vec3::new(-5.4, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.0, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.4, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.0, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 3.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 3.3, -5.0), Arc::clone(&ice)),
           //Arch - r7
           (Vec3::new(-5.4, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-5.0, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.0, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 3.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 3.7, -5.0), Arc::clone(&ice)),
           //Arch - r8
           (Vec3::new(-5.0, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(1.0, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 4.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 4.1, -5.0), Arc::clone(&ice)),
           //Arch - r9
           (Vec3::new(-5.0, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.6, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 4.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 4.5, -5.0), Arc::clone(&ice)),
           //Arch - r10
           (Vec3::new(-4.6, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.6, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 4.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 4.9, -5.0), Arc::clone(&ice)),
           //Arch - r11
           (Vec3::new(-4.6, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(0.2, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 5.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 5.7, -5.0), Arc::clone(&ice)),
           //Arch - r12
           (Vec3::new(0.2, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-4.2, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 5.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 5.7, -5.0), Arc::clone(&ice)),
           //Arch - r13
           (Vec3::new(-4.2, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.2, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 6.1, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 6.1, -5.0), Arc::clone(&ice)),
           //Arch - r14
           (Vec3::new(-0.2, 6.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.8, 6.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-0.6, 6.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 6.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 6.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 6.5, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 6.5, -5.0), Arc::clone(&ice)),
           //Arch - r15
           (Vec3::new(-0.6, 6.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 6.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 6.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 6.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 6.9, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 6.9, -5.0), Arc::clone(&ice)),
           //Arch - r16
           (Vec3::new(-0.6, 7.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.0, 7.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 7.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 7.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 7.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 7.3, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 7.3, -5.0), Arc::clone(&ice)),
           //Arch - r17
           (Vec3::new(-1.0, 7.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.4, 7.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-1.8, 7.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.2, 7.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-2.6, 7.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.0, 7.7, -5.0), Arc::clone(&ice)),
           (Vec3::new(-3.4, 7.7, -5.0), Arc::clone(&ice)),
    ];

    let mut objects: Vec<Arc<dyn RayIntersect + Send + Sync>> = Vec::new();

    for &(center, ref material) in &cube_data {
        let cube = Arc::new(Cube {
            center,
            size: 0.4,
            material: Arc::clone(material),
        }) as Arc<dyn RayIntersect + Send + Sync>;
        objects.push(cube);
    }

    let ground = Arc::new(Square {
        center: Vec3::new(-2.0, 0.7, -4.0), 
        size: 9.6, 
        material: Arc::clone(&snow),
    }) as Arc<dyn RayIntersect + Send + Sync>;

    objects.push(ground);
    

    objects
}
