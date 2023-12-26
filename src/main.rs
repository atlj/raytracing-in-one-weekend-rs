use std::{
    cell::RefCell,
    io::{stderr, IsTerminal},
    path::Path,
    rc::Rc,
    sync::Mutex,
};

use kdam::{
    term::{self, Colorizer},
    tqdm, BarExt, Column, RichProgress, Spinner,
};
use rand::{rngs::SmallRng, SeedableRng};
use raytracing_in_one_weekend_rust::{
    camera::{Camera, HittableVector},
    hittable::Sphere,
    material::{Glass, Glossy, Lambertian},
    vec3::Vec3,
};

const WIDTH: usize = 400;
const HEIGHT: usize = 225;

const SAMPLE_COUNT: usize = 50;
const REFLECTION_LIMIT: usize = 50;

const FOCAL_LENGTH: f64 = 1.0;

const SEED: u64 = 234235;

const TOTAL_RAYS: usize = WIDTH * HEIGHT * SAMPLE_COUNT;

fn main() {
    term::init(stderr().is_terminal());
    let _ = term::hide_cursor();

    let progress_bar = Mutex::new(RichProgress::new(
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
    ));

    let _ = progress_bar
        .lock()
        .unwrap()
        .write("Rendering...".colorize("bold yellow"));

    let camera_center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let rng = SmallRng::seed_from_u64(SEED);

    let camera = Camera {
        rng,
        camera_center,
        width: WIDTH as f64,
        height: HEIGHT as f64,
        sample_count: SAMPLE_COUNT,
        focal_length: FOCAL_LENGTH,
        reflection_limit: REFLECTION_LIMIT,
        output_path: Path::new("output.png"),
        hittables: create_hittables(),
    };

    let processed_rays: RefCell<usize> = RefCell::new(0);
    camera.render(|| {
        let mut processed_rays = processed_rays.borrow_mut();
        *processed_rays += 1;

        let _ = progress_bar.lock().unwrap().update_to(*processed_rays);
    });

    let _ = progress_bar
        .lock()
        .unwrap()
        .write("Render completed".colorize("bold green"));
}

fn create_hittables() -> HittableVector {
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

    vec![
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
    ]
}
