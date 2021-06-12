pub fn rotate_image(image_file: &str, num_rotations: u32, output_dir: Option<&str>) {
    println!("input image = {}", image_file);
    println!("num rotations = {}", num_rotations);

    if let Some(s) = output_dir {
        println!("output directory = {}", s);
    };
}