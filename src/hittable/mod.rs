use std::{ops::RangeInclusive, rc::Rc};

use crate::{aabb::AxisAlignedBoundingBox, material::Material, ray::Ray, vec3::Vec3};

mod bounding_volume;
mod sphere;

pub use bounding_volume::BoundingVolume;
pub use sphere::{CenterPosition, Sphere};

pub struct HitRecord {
    pub position: Vec3,
    pub normal: Vec3,
    pub multiplier: f64,
    pub did_hit_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, multiplier_range: RangeInclusive<f64>) -> Option<HitRecord>;
    fn get_bounding_box(&self) -> AxisAlignedBoundingBox;
    fn material(&self) -> &Rc<dyn Material>;
}
