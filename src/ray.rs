use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, multiplier: f64) -> Vec3 {
        self.origin + self.direction * multiplier
    }
}
