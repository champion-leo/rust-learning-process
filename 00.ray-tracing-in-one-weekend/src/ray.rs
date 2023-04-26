use crate::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod test_ray {
    use super::*;

    #[test]
    fn test_at() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(r.at(0.0), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(r.at(1.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(r.at(2.0), Vec3::new(2.0, 4.0, 6.0));
    }
}
