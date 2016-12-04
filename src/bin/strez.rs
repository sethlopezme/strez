use std::fs;
use std::io;
use std::io::Write;
use std::process;

extern crate strez;
//use strez::parse::Parse;
use strez::settings::Settings;

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

    match run(args) {
        Err(e) => {
            let _ = writeln!(&mut io::stderr(), "Error: {}", e);
            process::exit(1);
        },
        Ok(_) => process::exit(0),
    }
}

fn run(args: clap::ArgMatches) -> Result<(), String> {
    let settings = Settings::from_clap(&args)?;

    println!("args = {:#?}", args);
    println!("settings = {:#?}", settings);

    // if the "input" argument is present, get a file reader
//    let reader: Box<io::Read> = match args.value_of("infile") {
//        Some(path) => Box::new(fs::File::open(path).expect("Error: unable to open file")),
//        None       => panic!("Error: no input file provided"),
//    };

//    let source_format = args.value_of("format").unwrap_or("csv");
    // load data using the appropriate loader
    match settings.format {
        //        "csv" => strez::CSV::from_reader(reader),
        f     => return Err(String::from(format!("unsupported format '{}'", f))),
    };

    Ok(())
}
