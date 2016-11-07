use std::fs;
use std::io;

extern crate strez;
use strez::serialize::Deserialize;

#[macro_use]
extern crate clap;

fn main() {
    // load the yaml usage definition
    let yaml = load_yaml!("../usage/en.yml");
    // parse the command-line args from the yaml usage definition
    let args = clap::App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    
    // if the "input" argument is present, get a file reader
    let reader: Box<io::Read> = match args.value_of("infile") {
        Some(path) => Box::new(fs::File::open(path).expect("Error: unable to open file")),
        None       => panic!("Error: no input file provided"),
    };
    
    let source_format = args.value_of("format").unwrap_or("csv");
    // load data using the appropriate loader
    match source_format {
//        "csv" => strez::CSV::from_reader(reader),
        _     => panic!("Error: unsupported format '{}'", source_format),
    };
}
