use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Typester")
        .author("Jesus Bossa, x@jesusbossa.dev")
        .version("1.0.0")
        .about("Explains in brief what the program does")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("The Rust file to process (including extension)"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .help("The name of the Typescript file to output (including extension)"),
        )
        .get_matches();

    let input_filename = matches.get_one::<String>("input").expect("input required");
    let output_filename = matches
        .get_one::<String>("output")
        .expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);
}
