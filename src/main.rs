use clap::{Arg, Command};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn main() {
    let argument_required = "Argument required";
    let input_argument: Arg = Arg::new("input")
        .short('i')
        .long("input")
        .required(true)
        .help("The Rust file to process (including extension)");
    let ouput_argument: Arg = Arg::new("output")
        .short('o')
        .long("output")
        .required(true)
        .help("The name of the Typescript file to output (including extension)");

    let matches = Command::new("Typester")
        .author("Jesus Bossa, x@jesusbossa.dev")
        .version("1.0.0")
        .about("Convert Rust types to Typescript types")
        .arg(input_argument)
        .arg(ouput_argument)
        .get_matches();

    let input_filename = matches.get_one::<String>("input").expect(argument_required);
    let output_filename = matches
        .get_one::<String>("output")
        .expect(argument_required);

    let input_path = Path::new(input_filename);

    let mut input_file =
        File::open(input_path).expect(&format!("Unable to open file {}", input_path.display()));

    let mut input_file_text = String::new();

    input_file
        .read_to_string(&mut input_file_text)
        .expect("Unable to read file");

    // This is our tokenized version of Rust file ready to process
    let input_syntax: syn::File = syn::parse_file(&input_file_text).expect("Unable to parse file");

    // This string will store the output of the Typescript file that we will
    // continuously append to as we process the Rust file
    let mut output_text = String::new();

    for item in input_syntax.items.iter() {
        match item {
            // This `Item::Type` enum variant matches our type alias
            syn::Item::Type(item_type) => {
                let type_text = parse_item_type(item_type);
                output_text.push_str(&type_text);
            }
            _ => {
                dbg!("Encountered an unimplemented type");
            }
        }
    }

    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", output_text).expect("Failed to write to output file");

    dbg!(input_filename);
    dbg!(output_filename);
    dbg!(input_syntax);
}

fn parse_item_type(item_type: &syn::ItemType) -> String {
    String::from("todo")
}
