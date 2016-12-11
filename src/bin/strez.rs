use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;

extern crate strez;
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
    // exit with the error and code if an error is returned
    if let Err(ref e) = run(args) {
        let _ = writeln!(&mut io::stderr(), "Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: clap::ArgMatches) -> strez::Result<()> {
    let settings = Settings::from_clap(&args)?;
    let file = fs::File::open(&settings.infile).map_err(|e| format!("{} could not be read, \"{}\"", e.description(), settings.infile.display()))?;

//    println!("settings = {:#?}", settings);

    // load data using the appropriate loader
    match settings.format {
        //        "csv" => strez::CSV::from_reader(reader),
        format => return Err(format!("{} format is not yet supported", format)),
    };

    Ok(())
}
