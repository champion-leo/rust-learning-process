#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    values: (f64, f64, f64),
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.values.0, self.values.1, self.values.2)
    }
}

#[cfg(test)]
mod tests_vec3_display {
    use super::*;

    #[test]
    fn test_vec3_display() {
        let v = Vec3 {
            values: (1.1, 2.0, 3.0),
        };
        assert_eq!(format!("{}", v), "1.1 2 3");
    }
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
mod tests_vec3_std_ops_self {
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

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            values: (
                self.values.0 + _rhs.values.0,
                self.values.1 + _rhs.values.1,
                self.values.2 + _rhs.values.2,
            ),
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            values: (
                self.values.0 - _rhs.values.0,
                self.values.1 - _rhs.values.1,
                self.values.2 - _rhs.values.2,
            ),
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            values: (
                self.values.0 * _rhs.values.0,
                self.values.1 * _rhs.values.1,
                self.values.2 * _rhs.values.2,
            ),
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            values: (
                self.values.0 * _rhs,
                self.values.1 * _rhs,
                self.values.2 * _rhs,
            ),
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        (1.0 / _rhs) * self
    }
}

#[cfg(test)]
mod tests_vec3_std_ops {
    use super::*;

    #[test]
    fn test_add() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(
            v + Vec3 {
                values: (1.0, 1.0, 1.0),
            },
            Vec3 {
                values: (2.0, 3.0, 4.0),
            }
        );
    }

    #[test]
    fn test_sub() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(
            v - Vec3 {
                values: (1.0, 1.0, 1.0),
            },
            Vec3 {
                values: (0.0, 1.0, 2.0),
            }
        );
    }

    #[test]
    fn test_mul() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(
            v * Vec3 {
                values: (2.0, 2.0, 2.0),
            },
            Vec3 {
                values: (2.0, 4.0, 6.0),
            }
        );
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(
            v * 2.0,
            Vec3 {
                values: (2.0, 4.0, 6.0),
            }
        );
    }

    #[test]
    fn test_mul_scalar_reverse() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(2.0 * v, v * 2.0,);
    }

    #[test]
    fn test_div() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(
            v / 2.0,
            Vec3 {
                values: (0.5, 1.0, 1.5),
            }
        );
    }
}


pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.values.0 * v2.values.0 + v1.values.1 * v2.values.1 + v1.values.2 * v2.values.2
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        values: (
            v1.values.1 * v2.values.2 - v1.values.2 * v2.values.1,
            v1.values.2 * v2.values.0 - v1.values.0 * v2.values.2,
            v1.values.0 * v2.values.1 - v1.values.1 * v2.values.0,
        ),
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[cfg(test)]
mod tests_vec3 {
    use super::*;

    #[test]
    fn test_dot() {
        let v1 = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        let v2 = Vec3 {
            values: (2.0, 3.0, 4.0),
        };
        assert_eq!(dot(v1, v2), 20.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        let v2 = Vec3 {
            values: (2.0, 3.0, 4.0),
        };
        assert_eq!(
            cross(v1, v2),
            Vec3 {
                values: (-1.0, 2.0, -1.0),
            }
        );
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3 {
            values: (1.0, 2.0, 3.0),
        };
        assert_eq!(
            unit_vector(v),
            Vec3 {
                values: (0.2672612419124244, 0.5345224838248488, 0.8017837257372732),
            }
        );
    }
}