use image::RgbImage;
use raytracing_in_one_weekend_rust::{
    hittable::{Hittable, Sphere},
    ray::Ray,
    vec3::Vec3,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = WIDTH as f64 / HEIGHT as f64 * VIEWPORT_HEIGHT;

const FOCAL_LENGTH: f64 = 1.0;

const COLOR_WHITE: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

const COLOR_BLUE: Vec3 = Vec3 {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

type HittableVector = Vec<Box<dyn Hittable>>;

fn ray_color(ray: &Ray, hittables: &HittableVector) -> Vec3 {
    let closest_hit_record = hittables
        .iter()
        .flat_map(|hittable| hittable.hit(ray, 0.0, f64::INFINITY))
        .min_by_key(|hit_record| hit_record.multiplier as i64);

    if let Some(closest_hit_record) = closest_hit_record {
        return ((closest_hit_record.normal + 1.0) / 2.0) * 255.0;
    }

    let unit_direction = ray.direction.unit();

    let height_ratio = (unit_direction.y + 1.0) / 2.0;

    ((1.0 - height_ratio) * COLOR_WHITE + (height_ratio) * COLOR_BLUE) * 255.0
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

    let hittables: HittableVector = vec![
        Box::new(Sphere {
            center_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        }),
        Box::new(Sphere {
            center_position: Vec3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        }),
        Box::new(Sphere {
            center_position: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        }),
        Box::new(Sphere {
            center_position: Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
        }),
    ];

    for (x, y, color) in img.enumerate_pixels_mut() {
        let pixel_location = first_pixel_location
            + x as f64 * pixel_delta_horizontal
            + y as f64 * pixel_delta_vertical;

        let ray_direction = pixel_location - camera_center;
        let ray = Ray {
            origin: camera_center,
            direction: ray_direction,
        };

        *color = ray_color(&ray, &hittables).into();
    }

    img.save("output.png").unwrap();
}
