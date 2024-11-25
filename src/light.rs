use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::Intersect;
use crate::ray_intersect::{RayIntersect};

const ORIGIN_BIAS: f32 = 1e-4;

pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}

pub fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    let (n_cosi, eta, n_normal);
    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }
    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

pub fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let light_distance = (light.position - intersect.point).magnitude();
    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    let mut shadow_intensity = 0.0;
    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            let distance_ratio = shadow_intersect.distance / light_distance;
            shadow_intensity = 1.0 - distance_ratio.powf(2.0).min(1.0);
            break;
        }
    }
    shadow_intensity
}

pub fn calculate_uv(normal: Vec3, point: Vec3, cube_size: f32) -> (f32, f32) {
    let u = ((point.x / cube_size) + 0.5) % 1.0;
    let v = ((point.y / cube_size) + 0.5) % 1.0;
    (u.abs(), v.abs())
}


impl Light {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }
}
