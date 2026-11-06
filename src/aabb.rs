use std::{
    iter::Sum,
    ops::{Add, RangeInclusive},
};

use crate::ray::Ray;

type Interval = RangeInclusive<f64>;

#[derive(Debug, Clone)]
pub struct AxisAlignedBoundingBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AxisAlignedBoundingBox {
    const ZERO: AxisAlignedBoundingBox = AxisAlignedBoundingBox {
        x: 0.0..=0.0,
        y: 0.0..=0.0,
        z: 0.0..=0.0,
    };

    pub fn does_hit(&self, ray: &Ray, ray_interval: Interval) -> bool {
        let Some(x_intersection) =
            Self::axis_intersection(ray.origin.x, ray.direction.x, &self.x, &ray_interval)
        else {
            return false;
        };

        let Some(y_intersection) =
            Self::axis_intersection(ray.origin.y, ray.direction.y, &self.y, &x_intersection)
        else {
            return false;
        };

        Self::axis_intersection(ray.origin.z, ray.direction.z, &self.z, &y_intersection).is_some()
    }

    fn axis_intersection(
        component_origin: f64,
        component_direction: f64,
        axis_interval: &Interval,
        valid_interval: &Interval,
    ) -> Option<Interval> {
        let inverse_direction = 1.0 / component_direction;
        if inverse_direction.is_infinite() || inverse_direction.is_nan() {
            return None;
        }

        let intersection_a = (axis_interval.start() - component_origin) * inverse_direction;
        let intersection_b = (axis_interval.end() - component_origin) * inverse_direction;

        let intersection_min = Self::min_float(intersection_a, intersection_b);
        let intersection_max = Self::max_float(intersection_a, intersection_b);

        let final_intersection_min = Self::max_float(intersection_min, *valid_interval.start());
        let final_intersection_max = Self::min_float(intersection_max, *valid_interval.end());

        if final_intersection_min >= final_intersection_max {
            return None;
        }

        Some(final_intersection_min..=final_intersection_max)
    }

    fn min_float(a: f64, b: f64) -> f64 {
        match a.total_cmp(&b) {
            std::cmp::Ordering::Less => a,
            std::cmp::Ordering::Equal => a,
            std::cmp::Ordering::Greater => b,
        }
    }

    fn max_float(a: f64, b: f64) -> f64 {
        match a.total_cmp(&b) {
            std::cmp::Ordering::Greater => a,
            std::cmp::Ordering::Equal => a,
            std::cmp::Ordering::Less => b,
        }
    }
}

impl Add<AxisAlignedBoundingBox> for AxisAlignedBoundingBox {
    type Output = AxisAlignedBoundingBox;

    fn add(self, b: AxisAlignedBoundingBox) -> Self::Output {
        AxisAlignedBoundingBox {
            x: (Self::min_float(*self.x.start(), *b.x.start()))
                ..=(Self::max_float(*self.x.end(), *b.x.end())),
            y: (Self::min_float(*self.y.start(), *b.y.start()))
                ..=(Self::max_float(*self.y.end(), *b.y.end())),
            z: (Self::min_float(*self.z.start(), *b.z.start()))
                ..=(Self::max_float(*self.z.end(), *b.z.end())),
        }
    }
}

impl Sum for AxisAlignedBoundingBox {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, curr| acc + curr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{aabb::AxisAlignedBoundingBox, ray::Ray, vec3::Vec3};

    #[test]
    fn test_intersection() {
        let bounding_box = AxisAlignedBoundingBox {
            x: (1.0..=2.0),
            y: (1.0..=2.0),
            z: (1.0..=2.0),
        };

        assert!(bounding_box.does_hit(
            &Ray {
                direction: Vec3 {
                    x: 1.5,
                    y: 1.5,
                    z: 1.5
                }
                .unit(),
                origin: Vec3::ZERO,
                time: 0.0
            },
            0.0..=f64::INFINITY
        ));

        assert!(!bounding_box.does_hit(
            &Ray {
                direction: Vec3 {
                    x: -1.5,
                    y: 1.5,
                    z: 1.5
                }
                .unit(),
                origin: Vec3::ZERO,
                time: 0.0
            },
            0.0..=f64::INFINITY
        ));

        assert!(!bounding_box.does_hit(
            &Ray {
                direction: Vec3::ZERO,
                origin: Vec3::ZERO,
                time: 0.0
            },
            0.0..=f64::INFINITY
        ));

        assert!(bounding_box.does_hit(
            &Ray {
                direction: Vec3 {
                    x: 5.0,
                    y: 1.0,
                    z: 1.0,
                }
                .unit(),
                origin: Vec3 {
                    x: 0.9,
                    y: 0.9,
                    z: 0.9
                },
                time: 0.0
            },
            0.0..=f64::INFINITY
        ));

        assert!(bounding_box.does_hit(
            &Ray {
                direction: Vec3 {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                }
                .unit(),
                origin: Vec3 {
                    x: 4.0,
                    y: 4.0,
                    z: 4.0
                },
                time: 0.0
            },
            0.0..=f64::INFINITY
        ));

        assert!(!bounding_box.does_hit(
            &Ray {
                direction: Vec3 {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                }
                .unit(),
                origin: Vec3::ZERO,
                time: 0.0
            },
            0.0..=f64::INFINITY
        ));
    }
}
