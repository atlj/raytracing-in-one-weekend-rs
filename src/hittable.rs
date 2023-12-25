use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub position: Vec3,
    pub normal: Vec3,
    pub multiplier: f64,
    pub did_hit_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, minimum_multiplier: f64, maximum_multiplier: f64)
        -> Option<HitRecord>;
}

pub struct Sphere {
    pub center_position: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        minimum_multiplier: f64,
        maximum_multiplier: f64,
    ) -> Option<HitRecord> {
        let camera_to_sphere = ray.origin - self.center_position;
        let a = ray.direction.length_squared();
        let half_b = camera_to_sphere.dot(ray.direction);
        let c = camera_to_sphere.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrted = discriminant.sqrt();

        let root_1 = (-half_b - discriminant_sqrted) / a;

        let root = if root_1 >= minimum_multiplier && root_1 <= maximum_multiplier {
            root_1
        } else {
            (-half_b + discriminant_sqrted) / a
        };

        let position = ray.at(root);

        let normal = (position - self.center_position) / self.radius;
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
        return Some(hit_record);
    }
}
