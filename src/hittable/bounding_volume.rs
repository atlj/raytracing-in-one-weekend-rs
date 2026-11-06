use std::{ops::RangeInclusive, rc::Rc};

use crate::{aabb::AxisAlignedBoundingBox, hittable::Hittable, ray::Ray};

pub struct BoundingVolume {
    aabb: AxisAlignedBoundingBox,
    right: Rc<dyn Hittable>,
    left: Rc<dyn Hittable>,
}

impl Hittable for BoundingVolume {
    fn hit(
        &self,
        ray: &Ray,
        multiplier_range: RangeInclusive<f64>,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.aabb.does_hit(ray, multiplier_range.clone()) {
            return None;
        }

        let left_hit = self.left.hit(ray, multiplier_range.clone());
        let right_hit = self.right.hit(
            ray,
            *multiplier_range.start()
                ..=left_hit
                    .as_ref()
                    .map(|hit| hit.multiplier)
                    .unwrap_or(*multiplier_range.end()),
        );

        left_hit.or(right_hit)
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.aabb.clone()
    }

    fn material(&self) -> &Rc<dyn crate::material::Material> {
        unimplemented!()
    }
}
