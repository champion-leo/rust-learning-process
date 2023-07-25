use crate::{vec3::{Vec3, reflect, unit_vector, dot}, ray::Ray, object::HitRecord};

use super::Scatterable;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo , fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}