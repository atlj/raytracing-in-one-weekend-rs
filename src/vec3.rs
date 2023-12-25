use std::{iter::Sum, ops::RangeInclusive};

use image::Rgb;
use rand::{rngs::SmallRng, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn random(range: RangeInclusive<f64>, rng: &mut SmallRng) -> Vec3 {
        Vec3 {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
        loop {
            let random_vec = Vec3::random(-1.0..=1.0, rng);

            if random_vec.length_squared() < 1.0 {
                return random_vec;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut SmallRng) -> Vec3 {
        Vec3::random_in_unit_sphere(rng).unit()
    }

    pub fn random_on_hemisphere(normal: Vec3, rng: &mut SmallRng) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);

        if in_unit_sphere.dot(normal) >= 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn is_near_zero(&self) -> bool {
        let really_small_number = 1e-8;
        return self.x.abs() < really_small_number
            && self.y.abs() < really_small_number
            && self.z.abs() < really_small_number;
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Into<Rgb<u8>> for Vec3 {
    fn into(self) -> Rgb<u8> {
        Rgb([self.x as u8, self.y as u8, self.z as u8])
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut acc = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for item in iter {
            acc = acc + item
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn test_unit() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }
            .unit(),
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }
        );
    }
}
