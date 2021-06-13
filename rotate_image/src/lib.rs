use image::{self, DynamicImage, GenericImageView, Rgb};
use imageproc::{
    definitions::HasBlack,
    geometric_transformations::{self, Interpolation}
};

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

    let buf = match img {
        DynamicImage::ImageRgb8(buf) => Some(buf),
        _ => None
    };

    if let Some(buf) = buf {
        println!("Got an image buffer");
        for i in 0..num_rotations {
            let degrees = 360 * i / num_rotations;
            let radians = (degrees as f32).to_radians();
            println!("Angle = {} {}", degrees, radians);

            let rotatedImage = geometric_transformations::rotate_about_center(
                &buf, radians, Interpolation::Bilinear, Rgb::black()
            );
            //img.save("test.png").unwrap();
        }
    }
}

pub fn error(msg: &str) {
    println!("Error: {}", msg);
}