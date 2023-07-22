use crate::{ray::Ray, object::HitRecord, vec3::{Vec3, reflect, dot, unit_vector}};

pub trait Scatterable: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

// Mate object with Lambertian material
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}