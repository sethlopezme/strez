use std::env;

#[macro_use]
extern crate clap;

fn main() {
    let yaml = load_yaml!("cli/cli-en.yml");
    let args = clap::App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
}
