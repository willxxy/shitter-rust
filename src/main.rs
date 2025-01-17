use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use image::io::Reader as ImageReader;
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
use rand::Rng;
use std::path::Path;

fn read_image(path: &Path) -> DynamicImage {
    ImageReader::open(path).unwrap().decode().unwrap()
}

fn rotate_image(image: &DynamicImage, angle: f64) -> RgbaImage {
    let image = image.to_rgba8();
    rotate_about_center(&image, angle.to_radians() as f32, Interpolation::Bilinear, Rgba([0, 0, 0, 0]))
}

fn main() {
    let input_path = Path::new("test.png");
    let shit_image_path = Path::new("shit.png");
    let output_path = Path::new("out.png");

    let mut rng = rand::thread_rng();
    let mut input_image = read_image(&input_path);
    let shit_image = read_image(&shit_image_path);

    let (width, height) = input_image.dimensions();
    let num_shits = rng.gen_range(0..100);

    for _ in 0..num_shits {
        let x = rng.gen_range(0..width) as i64;
        let y = rng.gen_range(0..height) as i64;

        let resized_shit = shit_image.thumbnail(100, 100);

        let angle = rng.gen_range(0.0..360.0);
        let rotated_shit = rotate_image(&resized_shit, angle);

        image::imageops::overlay(&mut input_image, &DynamicImage::ImageRgba8(rotated_shit), x, y);
    }

    input_image.save(output_path).unwrap();
}
