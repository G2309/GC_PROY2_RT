use nalgebra_glm::Vec3;
use std::rc::Rc;
use crate::material::Material;
use crate::ray_intersect::{RayIntersect, Intersect};

pub struct Square {  
    pub center: Vec3,
    pub size: f32, 
    pub material: Rc<Material>,
}

impl Square {
    fn is_point_inside(&self, point: &Vec3) -> bool {
        let half_size = self.size / 2.0;
        (point.x >= self.center.x - half_size && point.x <= self.center.x + half_size) &&
        (point.z >= self.center.z - half_size && point.z <= self.center.z + half_size)
    }
}

impl RayIntersect for Square {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let normal = Vec3::new(0.0, 1.0, 0.0); 
        let d = -self.center.y; 
        let denom = nalgebra_glm::dot(&normal, &ray_direction);

        if denom.abs() > 1e-6 {
            let t = -(nalgebra_glm::dot(&normal, &ray_origin) + d) / denom;
            if t >= 0.0 {
                let hit_point = ray_origin + ray_direction * t;

                if self.is_point_inside(&hit_point) {
                    let half_size = self.size / 2.0;
                    let u = (hit_point.x - self.center.x + half_size) / self.size;
                    let v = (hit_point.z - self.center.z + half_size) / self.size;

                    return Intersect { 
                        point: hit_point,
                        normal,
                        distance: t,
                        material: Rc::clone(&self.material), 
                        is_intersecting: true,
                        u, 
                        v,
                    };
                }
            }
        }

        Intersect {
            point: Vec3::new(0.0, 0.0, 0.0), 
            normal: Vec3::new(0.0, 0.0, 0.0),
            distance: f32::INFINITY,
            material: Rc::clone(&self.material), 
            is_intersecting: false, 
            u: 0.0, 
            v: 0.0,
        }
    }
}

