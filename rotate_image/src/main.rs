use clap::{Arg, App};

fn main() {
    let app = create_args_parser();

    let matches = app.get_matches();

    let num_rotations = match matches.value_of("num_rotations") {
        None => 12,
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => if n > 0 && n <= 369 {
                    n
                } else {
                    return error("Rotation value out of range");
                }
                Err(_) => {
                    return error("Rotion must be a number")
                }
            }
        }
    };
    println!("num rotations = {}", num_rotations);

    let output_dir = matches.value_of("output_dir");
    if let Some(s) = output_dir {
        println!("output directory = {}", s);
    };

    let input_image = matches.value_of("INPUT").unwrap();
    println!("input image = {}", input_image);
}

fn create_args_parser() -> App<'static, 'static> {
    App::new("Simple image rotation")
        .version("0.1")
        .author("Erwin Bonsma")
        .about("A coding exercise")
        .arg(Arg::with_name("num_rotations")
            .short("r")
            .long("rotations")
            .value_name("NUM")
            .help("The number of rotations")
            .takes_value(true))
        .arg(Arg::with_name("output_dir")
            .short("o")
            .long("output")
            .value_name("PATH")
            .help("The directory where to create the rotated images")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("The input image")
            .required(true)
            .index(1))
}

fn error(msg: &str) {
    println!("Error: {}", msg);
}