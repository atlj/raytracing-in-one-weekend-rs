use std::path::Path;

use image::RgbImage;
use rand::{rngs::SmallRng, Rng};

use crate::{
    constants::{COLOR_BLACK, COLOR_BLUE, COLOR_WHITE, VIEWPORT_HEIGHT},
    hittable::Hittable,
    ray::Ray,
    vec3::Vec3,
};

pub struct Camera {
    pub hittables: HittableVector,
    pub focal_length: f64,
    pub width: f64,
    pub height: f64,
    pub reflection_limit: usize,
    pub sample_count: usize,
    pub output_path: &'static Path,
    pub camera_center: Vec3,
    pub rng: SmallRng,
}

impl Camera {
    pub fn render<F>(mut self, on_progress: F) -> ()
    where
        F: Fn() -> (),
    {
        let viewport_width = self.width / self.height * VIEWPORT_HEIGHT;

        let viewport_horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };

        let viewport_vertical = Vec3 {
            x: 0.0,
            y: -VIEWPORT_HEIGHT,
            z: 0.0,
        };

        let pixel_delta_horizontal = viewport_horizontal / self.width;
        let pixel_delta_vertical = viewport_vertical / self.height;

        let viewport_upperleft_location = Vec3 {
            x: -viewport_width / 2.0,
            y: VIEWPORT_HEIGHT / 2.0,
            z: -self.focal_length,
        };

        let mut img = RgbImage::new(self.width as u32, self.height as u32);

        let first_pixel_location =
            viewport_upperleft_location + (pixel_delta_vertical + pixel_delta_horizontal) / 2.0;

        for (x, y, color) in img.enumerate_pixels_mut() {
            let pixel_center = first_pixel_location
                + x as f64 * pixel_delta_horizontal
                + y as f64 * pixel_delta_vertical;

            let random_rays: Vec<Ray> = (0..self.sample_count)
                .map(|_| {
                    let random_x_offset =
                        pixel_delta_horizontal.x * (self.rng.gen_range(-0.5..=0.5) as f64);
                    let random_y_offset =
                        pixel_delta_vertical.y * (self.rng.gen_range(-0.5..=0.5) as f64);

                    let random_pixel_location = Vec3 {
                        x: pixel_center.x + random_x_offset,
                        y: pixel_center.y + random_y_offset,
                        z: pixel_center.z,
                    };

                    let ray_direction = random_pixel_location - self.camera_center;

                    Ray {
                        origin: self.camera_center,
                        direction: ray_direction,
                    }
                })
                .collect();

            let sum_of_samples: Vec3 = random_rays
                .iter()
                .map(|ray| {
                    on_progress();
                    self.ray_color(&ray, 0)
                })
                .sum();

            let mean_color = (sum_of_samples) / (self.sample_count as f64);

            let gamma_corrected_color = Vec3 {
                x: linear_to_gamma(mean_color.x),
                y: linear_to_gamma(mean_color.y),
                z: linear_to_gamma(mean_color.z),
            };

            *color = (gamma_corrected_color * 255.0).into()
        }

        img.save("output.png").unwrap();
    }

    fn ray_color(&mut self, ray: &Ray, reflection_count: usize) -> Vec3 {
        if reflection_count == self.reflection_limit {
            return COLOR_BLACK;
        };

        if let Some((hittable_index, closest_hit_record)) = self
            .hittables
            .iter()
            .enumerate()
            .flat_map(|(index, hittable)| Some((index, hittable.hit(ray, 0.001..=f64::INFINITY)?)))
            .min_by_key(|(_, hit_record)| hit_record.multiplier as i64)
        {
            // return (closest_hit_record.normal + COLOR_WHITE) / 2.0;

            let material = self.hittables[hittable_index].material();

            if let Some((attenuation, reflected_ray)) =
                material.scatter(&ray, &closest_hit_record, &mut self.rng)
            {
                return attenuation * self.ray_color(&reflected_ray, reflection_count + 1);
            }
        }

        let unit_direction = ray.direction.unit();

        let height_ratio = (unit_direction.y + 1.0) / 2.0;

        (1.0 - height_ratio) * COLOR_WHITE + (height_ratio) * COLOR_BLUE
    }
}

pub type HittableVector = Vec<Box<dyn Hittable>>;

fn linear_to_gamma(color: f64) -> f64 {
    color.sqrt()
}
