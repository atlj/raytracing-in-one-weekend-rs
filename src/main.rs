use std::{
    io::{stderr, IsTerminal},
    rc::Rc,
};

use image::RgbImage;
use kdam::{
    term::{self, Colorizer},
    tqdm, BarExt, Column, RichProgress, Spinner,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use raytracing_in_one_weekend_rust::{
    hittable::{Hittable, Sphere},
    material::{Glass, Glossy, Lambertian},
    ray::Ray,
    vec3::Vec3,
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 225;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = WIDTH as f64 / HEIGHT as f64 * VIEWPORT_HEIGHT;

const SAMPLE_COUNT: i32 = 50;

const FOCAL_LENGTH: f64 = 1.0;

const TOTAL_RAYS: usize = WIDTH as usize * HEIGHT as usize * SAMPLE_COUNT as usize;

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

const COLOR_BLACK: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

const SEED: u64 = 32420;

const REFLECTION_LIMIT: usize = 40;

type HittableVector = Vec<Box<dyn Hittable>>;

fn linear_to_gamma(color: f64) -> f64 {
    color.sqrt()
}

fn ray_color(
    ray: &Ray,
    hittables: &HittableVector,
    reflection_count: usize,
    rng: &mut SmallRng,
) -> Vec3 {
    if reflection_count == REFLECTION_LIMIT {
        return COLOR_BLACK;
    };

    if let Some((hittable_index, closest_hit_record)) = hittables
        .iter()
        .enumerate()
        .flat_map(|(index, hittable)| Some((index, hittable.hit(ray, 0.001..=f64::INFINITY)?)))
        .min_by_key(|(_, hit_record)| hit_record.multiplier as i64)
    {
        // return (closest_hit_record.normal + COLOR_WHITE) / 2.0;

        let material = hittables[hittable_index].material();

        if let Some((attenuation, reflected_ray)) = material.scatter(&ray, &closest_hit_record, rng)
        {
            return attenuation * ray_color(&reflected_ray, hittables, reflection_count + 1, rng);
        }
    }

    let unit_direction = ray.direction.unit();

    let height_ratio = (unit_direction.y + 1.0) / 2.0;

    (1.0 - height_ratio) * COLOR_WHITE + (height_ratio) * COLOR_BLUE
}

fn main() {
    let mut rng = SmallRng::seed_from_u64(SEED);
    term::init(stderr().is_terminal());
    let _ = term::hide_cursor();

    let mut progress_bar = RichProgress::new(
        tqdm!(total = TOTAL_RAYS, unit_scale = true, unit = "rays"),
        vec![
            Column::Spinner(Spinner::new(
                &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                80.0,
                1.0,
            )),
            Column::Animation,
            Column::Percentage(1),
            Column::Text("•".to_owned()),
            Column::CountTotal,
            Column::Text("•".to_owned()),
            Column::Rate,
            Column::Text("•".to_owned()),
            Column::RemainingTime,
        ],
    );
    let mut processed_ray_count: usize = 0;

    let _ = progress_bar.write("Rendering...".colorize("bold yellow"));

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

    let ground_material = Rc::new(Lambertian {
        albedo: Vec3 {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });

    let pink_material = Rc::new(Lambertian {
        albedo: Vec3 {
            x: 0.7,
            y: 0.3,
            z: 0.3,
        },
    });

    let gold_material = Rc::new(Glossy {
        albedo: Vec3 {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        rougness: 0.1,
    });

    let mirror_material = Rc::new(Glossy {
        albedo: Vec3 {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
        rougness: 0.02,
    });

    let glass_material = Rc::new(Glass {
        albedo: Vec3 {
            x: 0.97,
            y: 0.97,
            z: 0.97,
        },
        refractive_index: 1.5,
    });

    let hittables: HittableVector = vec![
        Box::new(Sphere {
            center_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.3,
            },
            radius: 0.5,
            mat: pink_material.clone(),
        }),
        Box::new(Sphere {
            center_position: Vec3 {
                x: -0.8,
                y: -0.3,
                z: -0.8,
            },
            radius: 0.2,
            mat: glass_material.clone(),
        }),
        Box::new(Sphere {
            center_position: Vec3 {
                x: -2.5,
                y: 0.3,
                z: -2.5,
            },
            radius: 0.8,
            mat: gold_material.clone(),
        }),
        Box::new(Sphere {
            center_position: Vec3 {
                x: 1.3,
                y: 0.3,
                z: -1.0,
            },
            radius: 0.8,
            mat: mirror_material.clone(),
        }),
        Box::new(Sphere {
            center_position: Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
            mat: ground_material.clone(),
        }),
    ];

    for (x, y, color) in img.enumerate_pixels_mut() {
        let pixel_center = first_pixel_location
            + x as f64 * pixel_delta_horizontal
            + y as f64 * pixel_delta_vertical;

        let random_rays: Vec<Ray> = (0..SAMPLE_COUNT)
            .map(|_| {
                let random_x_offset = pixel_delta_horizontal.x * (rng.gen_range(-0.5..=0.5) as f64);
                let random_y_offset = pixel_delta_vertical.y * (rng.gen_range(-0.5..=0.5) as f64);

                let random_pixel_location = Vec3 {
                    x: pixel_center.x + random_x_offset,
                    y: pixel_center.y + random_y_offset,
                    z: pixel_center.z,
                };

                let ray_direction = random_pixel_location - camera_center;

                Ray {
                    origin: camera_center,
                    direction: ray_direction,
                }
            })
            .collect();

        let sum_of_samples: Vec3 = random_rays
            .iter()
            .map(|ray| {
                processed_ray_count += 1;

                let _ = progress_bar.update_to(processed_ray_count);
                ray_color(&ray, &hittables, 0, &mut rng)
            })
            .sum();

        let mean_color = (sum_of_samples) / (SAMPLE_COUNT as f64);

        let gamma_corrected_color = Vec3 {
            x: linear_to_gamma(mean_color.x),
            y: linear_to_gamma(mean_color.y),
            z: linear_to_gamma(mean_color.z),
        };

        *color = (gamma_corrected_color * 255.0).into()
    }

    let _ = progress_bar.write("Render completed".colorize("bold green"));

    img.save("output.png").unwrap();
}
