use std::sync::Arc;

use crate::material::Scatterable;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};
use dyn_clonable::*;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Scatterable>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /**
     * Set the normal and 
     */
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[clonable]
pub trait Hittable: Send + Sync + Clone {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Scatterable>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Scatterable>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let mut res = HitRecord {
            p,
            normal,
            material: self.material.clone(),
            t,
            front_face: false,
        };
        let outward_normal = (p - self.center) / self.radius;
        res.set_face_normal(r, outward_normal);
        Some(res)
    }
}

#[cfg(test)]
mod sphere_tests {

    use crate::material::lambertian::Lambertian;

    use super::*;

    fn get_material() -> Arc<dyn Scatterable> {
        Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0)))
    }

    #[test]
    fn new() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        let sphere = Sphere::new(center, radius, get_material());
        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, radius);
    }

    #[test]
    fn hit() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1., get_material());
        let ray = Ray::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0));
        let hit_record = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit_record.is_some());
        let hit_record = hit_record.unwrap();
        assert_eq!(hit_record.p, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.t, 1.0);
        assert!(hit_record.front_face);
    }

    #[test]
    fn hit_from_far_away() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        let sphere = Sphere::new(center, radius, get_material());
        let ray = Ray::new(Vec3::new(0.0, 0.0, -100.0), Vec3::new(0.0, 0.0, 1.0));
        let hit_record = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit_record.is_some());
        let hit_record = hit_record.unwrap();
        assert_eq!(hit_record.p, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.t, 99.0);
        assert!(hit_record.front_face);
    }

    #[test]
    fn hit_from_inside() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1., get_material());
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.5));
        let hit_record = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit_record.is_some());
        let hit_record = hit_record.unwrap();
        assert_eq!(hit_record.p, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.t, 2.0);
        assert!(!hit_record.front_face);
    }

    #[test]
    fn hit_on_border() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        let sphere = Sphere::new(
            center,
            radius,
            get_material(),
        );
        let ray = Ray::new(Vec3::new(1.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 2.0));
        let hit_record = sphere.hit(&ray, 0.0, 100.0);
        assert!(hit_record.is_some());
        let hit_record = hit_record.unwrap();
        assert_eq!(hit_record.p, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(hit_record.t, 1.0);
    }
}
