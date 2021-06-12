use clap::{Arg, App};

fn main() {
    let app = create_args_parser();

    let matches = app.get_matches();

    println!("Parsed args...");
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
