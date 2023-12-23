use image::RgbImage;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn main() {
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let rel_x = x as f32 / WIDTH as f32;
        let rel_y = y as f32 / HEIGHT as f32;
        let r = (rel_x * 255.0) as u8;
        let g = (rel_y * 255.0) as u8;
        let b = (r / 2) + (g / 2);
        *pixel = image::Rgb([r, g, b]);
    }

    img.save("output.png").unwrap();
}
