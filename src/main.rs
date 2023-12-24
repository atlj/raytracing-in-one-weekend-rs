use image::RgbImage;
use raytracing_in_one_weekend_rust::vec3::Vec3;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn main() {
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let relative_x = x as f64 / WIDTH as f64;
        let relative_y = y as f64 / HEIGHT as f64;

        let color = Vec3 {
            x: relative_x * 255.0,
            y: relative_y * 255.0,
            z: (relative_x * 255.0 / 3.0) + (relative_y * 255.0 / 3.0),
        };

        *pixel = color.into();
    }

    img.save("output.png").unwrap();
}
