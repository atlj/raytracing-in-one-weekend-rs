use rand::rngs::SmallRng;

use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(Vec3, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord, rng: &mut SmallRng) -> Option<(Vec3, Ray)> {
        let direction = hit_record.normal + Vec3::random_unit_vector(rng);

        let scattered_ray = Ray {
            origin: hit_record.position,
            direction: if !direction.is_near_zero() {
                direction
            } else {
                hit_record.normal
            },
        };

        Some((self.albedo, scattered_ray))
    }
}
