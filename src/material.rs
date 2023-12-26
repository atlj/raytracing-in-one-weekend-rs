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

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Glossy {
    pub albedo: Vec3,
    pub rougness: f64,
}

pub struct Glass {
    pub albedo: Vec3,
    pub refractive_index: f64,
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

impl Material for Glossy {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(Vec3, Ray)> {
        let direction = incoming_ray.direction.reflect(hit_record.normal)
            + Vec3::random_unit_vector(rng) * self.rougness;

        let reflected_ray = Ray {
            origin: hit_record.position,
            direction,
        };

        return Some((self.albedo, reflected_ray));
    }
}

impl Material for Glass {
    fn scatter(
        &self,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        _: &mut SmallRng,
    ) -> Option<(Vec3, Ray)> {
        let refraction_ratio = if hit_record.did_hit_front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let direction_unit = incoming_ray.direction.unit();
        let refracted_direction = direction_unit.refract(&hit_record.normal, refraction_ratio);

        Some((
            self.albedo,
            Ray {
                origin: hit_record.position,
                direction: refracted_direction,
            },
        ))
    }
}
