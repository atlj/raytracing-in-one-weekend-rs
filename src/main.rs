use image::RgbImage;
use raytracing_in_one_weekend_rust::{ray::Ray, vec3::Vec3};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = WIDTH as f64 / HEIGHT as f64 * VIEWPORT_HEIGHT;

const FOCAL_LENGTH: f64 = 1.0;

fn hit_sphere(sphere_center: Vec3, sphere_radius: f64, ray: &Ray) -> Option<f64> {
    let camera_to_sphere = ray.origin - sphere_center;
    let a = ray.direction.dot(ray.direction);
    let b = camera_to_sphere.dot(ray.direction) * 2.0;
    let c = camera_to_sphere.dot(camera_to_sphere) - sphere_radius * sphere_radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    return Some((-b - discriminant.sqrt()) / (2.0 * a));
}

fn ray_color(ray: &Ray) -> Vec3 {
    let sphere_center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: FOCAL_LENGTH * -1.0,
    };

    if let Some(multiplier) = hit_sphere(sphere_center, 0.5, ray) {
        let intersection_point = ray.at(multiplier);
        let normal = intersection_point - sphere_center;

        let normal_unit = normal.unit();

        return ((normal_unit + 1.0) / 2.0) * 255.0;
    }

    let white = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let blue = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    let unit_direction = ray.direction.unit();

    let height_ratio = (unit_direction.y + 1.0) / 2.0;

    ((1.0 - height_ratio) * white + (height_ratio) * blue) * 255.0
}

fn main() {
    let camera_center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let viewport_horizontal = Vec3 {
        x: VIEWPORT_WIDTH,
        y: 0.0,
        z: 0.0,
    };

    let viewport_vertical = Vec3 {
        x: 0.0,
        y: -VIEWPORT_HEIGHT,
        z: 0.0,
    };

    let pixel_delta_horizontal = viewport_horizontal / WIDTH as f64;
    let pixel_delta_vertical = viewport_vertical / HEIGHT as f64;

    let viewport_upperleft_location = Vec3 {
        x: -VIEWPORT_WIDTH / 2.0,
        y: VIEWPORT_HEIGHT / 2.0,
        z: -FOCAL_LENGTH,
    };

    let first_pixel_location =
        viewport_upperleft_location + (pixel_delta_vertical + pixel_delta_horizontal) / 2.0;

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for (x, y, color) in img.enumerate_pixels_mut() {
        let pixel_location = first_pixel_location
            + x as f64 * pixel_delta_horizontal
            + y as f64 * pixel_delta_vertical;

        let ray_direction = pixel_location - camera_center;
        let ray = Ray {
            origin: camera_center,
            direction: ray_direction,
        };

        *color = ray_color(&ray).into();
    }

    img.save("output.png").unwrap();
}
