#[derive(Debug)]
struct Vec3 {
    values: (f64, f64, f64),
}

// Accessors
impl Vec3 {
    fn x(&self) -> &f64 {
        return &self.values.0;
    }

    fn y(&self) -> &f64 {
        return &self.values.1;
    }

    fn z(&self) -> &f64 {
        return &self.values.2;
    }
}

#[cfg(test)]
mod tests_vec3_accessors {
    use super::*;

    const V: Vec3 = Vec3 {
        values: (1.0, 2.0, 3.0),
    };

    #[test]
    fn test_x() {
        assert_eq!(*V.x(), 1.0);
    }

    #[test]
    fn test_y() {
        assert_eq!(*V.y(), 2.0);
    }

    #[test]
    fn test_z() {
        assert_eq!(*V.z(), 3.0);
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.values.0 == other.values.0
            && self.values.1 == other.values.1
            && self.values.2 == other.values.2
    }
}
#[cfg(test)]
mod tests_vec3_eq {
    use super::*;

    #[test]
    fn test_vec3_eq() {
        let v1 = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        let v2 = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_vec3_neq_x() {
        let v1 = Vec3 {
            values: (1.0, 1.0, 1.0),
        };
        let v2 = Vec3 {
            values: (2.0, 1.0, 1.0),
        };
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_vec3_neq_y() {
        let v1 = Vec3 {
            values: (1.0, 1.0, 1.0),
        };
        let v2 = Vec3 {
            values: (1.0, 2.0, 1.0),
        };
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_vec3_neq_z() {
        let v1 = Vec3 {
            values: (1.0, 1.0, 1.0),
        };
        let v2 = Vec3 {
            values: (1.0, 1.0, 2.0),
        };
        assert_ne!(v1, v2);
    }
}

impl Vec3 {
    fn length_squared(&self) -> f64 {
        self.values.0 * self.values.0
            + self.values.1 * self.values.1
            + self.values.2 * self.values.2
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

#[cfg(test)]
mod tests_vec3_length {
    use super::*;

    #[test]
    fn test_length_squared() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(v.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(v.length(), 14.0_f64.sqrt());
    }
}



use std::ops;

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            values: (-self.values.0, -self.values.1, -self.values.2),
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.values.0,
            1 => &self.values.1,
            2 => &self.values.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.values.0 += _rhs.values.0;
        self.values.1 += _rhs.values.1;
        self.values.2 += _rhs.values.2;
    }
}


impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        self.values.0 *= _rhs;
        self.values.1 *= _rhs;
        self.values.2 *= _rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

#[cfg(test)]
mod tests_vec3_std_ops {
    use super::*;
    #[test]
    fn test_neg() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(
            -v,
            Vec3 {
                values: (-1.0, -2.0, -3.0),
            }
        );
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        v += Vec3 {
            values: (1.0, 1.0, 1.0),
        };
        assert_eq!(
            v,
            Vec3 {
                values: (2.0, 3.0, 4.0),
            }
        );
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        v *= 2.0;
        assert_eq!(
            v,
            Vec3 {
                values: (2.0, 4.0, 6.0),
            }
        );
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        v /= 2.0;
        assert_eq!(
            v,
            Vec3 {
                values: (0.5, 1.0, 1.5),
            }
        );
    }

    #[test]
    fn test_index() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        v[3];
    }
}
