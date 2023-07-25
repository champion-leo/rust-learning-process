use crate::{ray::Ray, object::HitRecord, vec3::Vec3};

pub trait Scatterable: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;