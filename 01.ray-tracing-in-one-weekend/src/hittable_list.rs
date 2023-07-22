use std::sync::Arc;

use crate::{
    object::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                res = Some(hit);
            }
        }
        res
    }
}

#[cfg(test)]
mod hittable_list_test {
    use super::*;
    use crate::{material::Lambertian, object::Sphere, vec3::Vec3};

    #[test]
    fn test_new() {
        let list = HittableList::new();
        assert!(list.objects.is_empty());
    }

    #[test]
    fn test_add() {
        let mut list = HittableList::new();
        list.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        )));
        assert_eq!(list.objects.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut list = HittableList::new();
        list.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        )));
        assert!(list.objects.len() > 0);
        list.clear();
        assert!(list.objects.is_empty());
    }

    #[test]
    fn test_hit() {
        let mut list = HittableList::new();
        list.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        )));
        list.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        )));

        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let res = list.hit(&r, 0.0, 100.0);
        assert!(res.is_some());
        let hit = res.unwrap();
        assert_eq!(hit.t, 0.5);
        assert_eq!(hit.p, Vec3::new(0.0, 0.0, -0.5));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }
}
