use std::{ops::RangeInclusive, rc::Rc};

use crate::{aabb::AxisAlignedBoundingBox, hittable::Hittable, ray::Ray};

pub struct BoundingVolumeHierarchy {
    // TODO: maybe we should remove this?
    boxes: Vec<Rc<dyn Hittable>>,
    root: Option<Rc<dyn Hittable>>,
}

impl BoundingVolumeHierarchy {
    fn new(boxes: Vec<AxisAlignedBoundingBox>) -> Self {
        todo!()
    }
}
