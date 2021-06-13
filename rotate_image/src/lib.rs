use image::{self, DynamicImage, EncodableLayout, Pixel};
use imageproc::{
    definitions::{Clamp, Image, HasBlack},
    geometric_transformations::{self, Interpolation}
};
use conv::ValueInto;
use std::path::Path;

pub fn rotate_image(image_file: &str, num_rotations: u32, output_dir: Option<&str>) {
    let img = match image::open(image_file) {
        Ok(img) => img,
        Err(err) => {
            return error(&format!("Failed to read input image. {}", err)[..])
        }
    };

    // Convert path string to Path and check it exists
    let output_path = match output_dir {
        Some(path) => {
            let path = Path::new(path);
            if !path.exists() {
                return error("Output path does not exist");
            }
            Some(path)
        }
        None => None
    };

    match img {
        DynamicImage::ImageRgb8(buf) => rotate_image_buffer(&buf, num_rotations, output_path),
        DynamicImage::ImageRgba8(buf) => rotate_image_buffer(&buf, num_rotations, output_path),
        _ => error("Unsupported image type"),
    };
}

fn rotate_image_buffer<P>(img: &Image<P>, num_rotations: u32, output_path: Option<&Path>)
    where
        P: Pixel + Send + Sync + HasBlack + 'static,
        <P as Pixel>::Subpixel: Send + Sync,
        [P::Subpixel]: EncodableLayout,
        <P as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>
{
    for i in 0..num_rotations {
        let degrees = 360 * i / num_rotations;
        let radians = (degrees as f32).to_radians();

        let rotated_image = geometric_transformations::rotate_about_center(
            &img, radians, Interpolation::Bilinear, P::black()
        );

        if let Some(path) = output_path {
            let output_file = format!("output{:03}.png", degrees);
            let path = path.join(output_file);
            println!("Saving {:?}", path);

            rotated_image.save(path).unwrap();
        };        
    }
}


pub fn error(msg: &str) {
    println!("Error: {}", msg);
}