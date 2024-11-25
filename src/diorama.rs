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
    let crying_obsidian_texture = Rc::new(Texture::new("src/textures/crying_obsidian.png"));
    let redstone_lamp_texture = Rc::new(Texture::new("src/textures/redstone_lamp_on.png"));

    // Material
    let ice = Rc::new(Material::new(
        Color::new(37, 150, 190), 
        50.0, 
        [1.0, 0.6, 0.2, 0.3], 
        0.0, 
        Some(ice_texture), 
        Color::new(5,10,75),
        0.0
    ));
    let snow = Rc::new(Material::new(
        Color::new(255, 255, 255), 
        5.0, 
        [1.0, 0.1, 0.075, 0.0], 
        0.0, 
        Some(snow), 
        Color::new(0,0,0),
        0.0
    ));
    let crying_obsidian = Rc::new(Material::new(
        Color::new(43, 1, 120), 
        90.0, 
        [0.8, 0.5, 0.0, 0.0], 
        1.7, 
        Some(crying_obsidian_texture), 
        Color::new(131,8,228),
        2.0
    ));
    let  redstone_lamp = Rc::new(Material::new(
        Color::new(231, 183, 119), 
        5.0, 
        [0.9, 0.4, 0.0, 0.0], 
        0.0, 
        Some(redstone_lamp_texture), 
        Color::new(255,57,0),
        10.0
    ));


       let cube_data = [
            // portal
            (Vec3::new(-5.0, 0.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-5.0, 1.3, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-5.0, 1.7, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-5.0, 2.1, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-5.0, 2.5, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-5.0, 2.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.0, 0.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.0, 1.3, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.0, 1.7, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.0, 2.1, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.0, 2.5, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.0, 2.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-4.6, 2.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-4.2, 2.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.8, 2.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.4, 2.9, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-4.2, 3.3, -5.0), Rc::clone(&redstone_lamp)),
            (Vec3::new(-3.8, 3.3, -5.0), Rc::clone(&redstone_lamp)),
            
            // inner portal
            (Vec3::new(-4.6, 0.9, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.6, 1.3, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.6, 1.7, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.6, 2.1, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.6, 2.5, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.2, 0.9, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.2, 1.3, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.2, 1.7, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.2, 2.1, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-4.2, 2.5, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.8, 0.9, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.8, 1.3, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.8, 1.7, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.8, 2.1, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.8, 2.5, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.4, 0.9, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.4, 1.3, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.4, 1.7, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.4, 2.1, -5.0), Rc::clone(&crying_obsidian)),
            (Vec3::new(-3.4, 2.5, -5.0), Rc::clone(&crying_obsidian)),

           // Arch - r0
           (Vec3::new(2.6, 0.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-6.6, 0.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(2.2, 0.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-6.2, 0.9, -3.0), Rc::clone(&ice)),
           // Arch - r1
           (Vec3::new(2.6, 1.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-6.6, 1.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(2.2, 1.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-6.2, 1.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.8, 1.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.8, 1.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.4, 1.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.4, 1.3, -3.0), Rc::clone(&ice)),
           //Arch - r2
           (Vec3::new(2.2, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-6.2, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.8, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.8, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.8, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.8, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.4, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.4, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.0, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.0, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 1.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 1.7, -3.0), Rc::clone(&ice)),
           // Arch - r3
           (Vec3::new(-6.2, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.8, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.4, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.4, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.0, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.0, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 2.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 2.1, -3.0), Rc::clone(&ice)),
           // Arch - r4
           (Vec3::new(-5.8, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.4, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.0, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 2.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 2.5, -3.0), Rc::clone(&ice)),
           //Arch - r5
           (Vec3::new(-5.8, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.4, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.0, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.8, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.4, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 2.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 2.9, -3.0), Rc::clone(&ice)),
           //Arch - r6
           (Vec3::new(-5.4, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.0, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.4, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.0, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 3.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 3.3, -3.0), Rc::clone(&ice)),
           //Arch - r7
           (Vec3::new(-5.4, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-5.0, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.0, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 3.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 3.7, -3.0), Rc::clone(&ice)),
           //Arch - r8
           (Vec3::new(-5.0, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(1.0, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 4.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 4.1, -3.0), Rc::clone(&ice)),
           //Arch - r9
           (Vec3::new(-5.0, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.6, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 4.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 4.5, -3.0), Rc::clone(&ice)),
           //Arch - r10
           (Vec3::new(-4.6, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.6, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 4.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 4.9, -3.0), Rc::clone(&ice)),
           //Arch - r11
           (Vec3::new(-4.6, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(0.2, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 5.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 5.7, -3.0), Rc::clone(&ice)),
           //Arch - r12
           (Vec3::new(0.2, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-4.2, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 5.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 5.7, -3.0), Rc::clone(&ice)),
           //Arch - r13
           (Vec3::new(-4.2, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.2, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 6.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 6.1, -3.0), Rc::clone(&ice)),
           //Arch - r14
           (Vec3::new(-0.2, 6.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.8, 6.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-0.6, 6.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 6.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 6.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 6.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 6.5, -3.0), Rc::clone(&ice)),
           //Arch - r15
           (Vec3::new(-0.6, 6.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 6.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 6.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 6.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 6.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 6.9, -3.0), Rc::clone(&ice)),
           //Arch - r16
           (Vec3::new(-0.6, 7.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.0, 7.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 7.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 7.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 7.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 7.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 7.3, -3.0), Rc::clone(&ice)),
           //Arch - r17
           (Vec3::new(-1.0, 7.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.4, 7.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 7.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 7.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 7.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 7.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.4, 7.7, -3.0), Rc::clone(&ice)),
           //Arch - r18
           (Vec3::new(-1.4, 8.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 8.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 8.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 8.1, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 8.1, -3.0), Rc::clone(&ice)),
           //Arch - r18
           (Vec3::new(-1.4, 8.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-1.8, 8.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 8.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 8.5, -3.0), Rc::clone(&ice)),
           (Vec3::new(-3.0, 8.5, -3.0), Rc::clone(&ice)),
           //Arch - r18
           (Vec3::new(-1.8, 8.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 8.9, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 8.9, -3.0), Rc::clone(&ice)),
           //Arch - r18
           (Vec3::new(-1.8, 9.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 9.3, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.2, 9.7, -3.0), Rc::clone(&ice)),
           (Vec3::new(-2.6, 9.3, -3.0), Rc::clone(&ice)),
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
        size: 9.6, 
        material: Rc::clone(&snow),
    }) as Box<dyn RayIntersect>;

    objects.push(ground);
    

    objects
}
