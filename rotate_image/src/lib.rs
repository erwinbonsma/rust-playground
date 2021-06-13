use image::{self, DynamicImage, GenericImageView, Pixel, Rgb};
use imageproc::{
    definitions::{Clamp, Image, HasBlack},
    geometric_transformations::{self, Interpolation}
};
use conv::ValueInto;

pub fn rotate_image(image_file: &str, num_rotations: u32, output_dir: Option<&str>) {
    if let Some(s) = output_dir {
        println!("output directory = {}", s);
    };

    let img = match image::open(image_file) {
        Ok(img) => img,
        Err(err) => {
            return error(&format!("Failed to read input image. {}", err)[..])
        }
    };

    match img {
        DynamicImage::ImageRgb8(buf) => rotate_image_buffer(&buf, num_rotations, output_dir),
        _ => error("Unsupported image type"),
    };
}

fn rotate_image_buffer<P>(img: &Image<P>, num_rotations: u32, output_dir: Option<&str>)
    where
        P: Pixel + Send + Sync + HasBlack + 'static,
        <P as Pixel>::Subpixel: Send + Sync,
        <P as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>
{
    for i in 0..num_rotations {
        let degrees = 360 * i / num_rotations;
        let radians = (degrees as f32).to_radians();
        println!("Angle = {} {}", degrees, radians);

        let rotated_iomage = geometric_transformations::rotate_about_center(
            &img, radians, Interpolation::Bilinear, P::black()
        );
        //img.save("test.png").unwrap();
    }
}


pub fn error(msg: &str) {
    println!("Error: {}", msg);
}