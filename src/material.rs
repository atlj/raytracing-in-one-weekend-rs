use rand::rngs::SmallRng;

use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(f64, Ray)>;
}

#[derive(Clone)]
pub struct DiffuseMaterial;

impl Material for DiffuseMaterial {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord, rng: &mut SmallRng) -> Option<(f64, Ray)> {
        let direction = hit_record.normal + Vec3::random_unit_vector(rng);

        let scattered_ray = Ray {
            origin: hit_record.position,
            direction,
        };

        Some((0.5, scattered_ray))
    }
}
