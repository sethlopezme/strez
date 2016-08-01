use std::fs;
use std::io;

extern crate strez;
use strez::input::Reader;

#[macro_use]
extern crate clap;

fn main() {
    // load the yaml usage definition
    let yaml = load_yaml!("cli/cli-en.yml");
    // parse the command-line args from the yaml usage definition
    let args = clap::App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    
    // if the "input" argument is present, get a file reader
    let reader: Box<io::Read> = match args.value_of("input") {
        Some(path) => Box::new(fs::File::open(path).expect("Unable to open file.")),
        None       => Box::new(io::stdin()),
    };
    
    let source_format = args.value_of("source-format").unwrap_or("csv");
    // load data using the appropriate loader
    match source_format {
        "csv" => strez::CSV::input_from_reader(reader),
        _     => panic!("Unsupported source format: {}", source_format),
    };
}
