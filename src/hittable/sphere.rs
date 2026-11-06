use std::{ops::RangeInclusive, rc::Rc};

use crate::{
    aabb::AxisAlignedBoundingBox,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub enum CenterPosition {
    Static(Vec3),
    Moving((Vec3, Vec3)),
}

impl CenterPosition {
    fn at(&self, time: f64) -> Vec3 {
        match self {
            CenterPosition::Static(static_pos) => *static_pos,
            CenterPosition::Moving((a, b)) => Vec3::lerp(a, b, time),
        }
    }
}

pub struct Sphere {
    pub center_position: CenterPosition,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    fn get_bounding_box_at(&self, center: Vec3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            x: (center.x - self.radius)..=(center.x + self.radius),
            y: (center.y - self.radius)..=(center.y + self.radius),
            z: (center.z - self.radius)..=(center.z + self.radius),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, multiplier_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let current_center = self.center_position.at(ray.time);
        let camera_to_sphere = ray.origin - current_center;
        let a = ray.direction.length_squared();
        let half_b = camera_to_sphere.dot(ray.direction);
        let c = camera_to_sphere.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrted = discriminant.sqrt();

        let root_1 = (-half_b - discriminant_sqrted) / a;
        let root_2 = (-half_b + discriminant_sqrted) / a;

        let root = if multiplier_range.contains(&root_1) {
            root_1
        } else if multiplier_range.contains(&root_2) {
            root_2
        } else {
            return None;
        };

        let position = ray.at(root);

        let normal = (position - current_center) / self.radius;
        let (calculated_normal, did_hit_front_face) = if ray.direction.dot(normal) > 0.0 {
            (-normal, false)
        } else {
            (normal, true)
        };

        let hit_record = HitRecord {
            multiplier: root,
            position,
            normal: calculated_normal,
            did_hit_front_face,
        };

        Some(hit_record)
    }

    fn material(&self) -> &Rc<dyn Material> {
        &self.mat
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        match self.center_position {
            CenterPosition::Static(center) => self.get_bounding_box_at(center),
            CenterPosition::Moving((start, end)) => {
                self.get_bounding_box_at(start) + self.get_bounding_box_at(end)
            }
        }
    }
}
