use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        // Here I can't use const because my operators overloading are not const compatible
        // there might be a way to do it but I don't know it yet...
        let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner: Vec3 =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
