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
    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    let mut shadow_intensity = 0.0;
    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            let emmisive_intensity = 0.2;
            shadow_intensity += emmisive_intensity;
        }
        break;
    }
    shadow_intensity
}

pub fn calculate_uv(normal: Vec3, point: Vec3, cube_size: f32) -> (f32, f32) {
    let u = ((point.x / cube_size) + 0.5) % 1.0;
    let v = ((point.y / cube_size) + 0.5) % 1.0;
    (u.abs(), v.abs())
}

pub fn calculate_emissive_lighting(
    intersect: &Intersect,
    objects: &[Box<dyn RayIntersect>],
) -> Color {
    let mut total_emission = Color::new(0, 0, 0);
    for object in objects {
        let object_intersect = object.ray_intersect(&intersect.point, &intersect.normal);

        // Si el objeto es intersectado y es emisivo, sumamos su emisión.
        if object_intersect.is_intersecting {
            let material = &object_intersect.material;
            if material.emmisive_intensity > 0.0 {
                let direction_to_light = (object_intersect.point - intersect.point).normalize();
                let distance = (object_intersect.point - intersect.point).magnitude();

                // Atenuación basada en la distancia
                let attenuation = 1.0 / (distance * distance);

                // Contribución al color final
                let emission = material.emmisive_color * material.emmisive_intensity * attenuation;
                total_emission = total_emission + emission;
            }
        }
    }
    total_emission
}


pub fn calculate_lighting(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> Color {
    let mut lighting = Color::new(0, 0, 0);
    let shadow_intensity = cast_shadow(intersect, light, objects);
    if shadow_intensity > 0.0 {
        let light_dir = (light.position - intersect.point).normalize();
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);
        lighting = lighting + light.color * light.intensity * diffuse_intensity * shadow_intensity;
    }
    let emissive_lighting = calculate_emissive_lighting(intersect, objects);
    lighting = lighting + emissive_lighting;

    lighting
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
