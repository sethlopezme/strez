use std::error::Error;
use std::fs;
use std::path::PathBuf;

use platform::PlatformKind;
use language::LanguageKind;

extern crate clap;

#[derive(Debug)]
pub struct Settings<'a> {
    pub default_language: Option<LanguageKind>,
    pub force: bool,
    pub format: &'a str,
    pub infile: PathBuf,
    pub outdir: PathBuf,
    pub platform: Vec<PlatformKind>,
}

impl<'a> Settings<'a> {
    /*
     * Takes argument matches from clap and turns them into a Settings struct that is easier to use.
     */
    pub fn from_clap(args: &'a clap::ArgMatches) -> Result<Settings<'a>, String> {
        let settings = Settings {
            default_language: match args.value_of("default_language") {
                Some(language_code) => Some(language_code.parse::<LanguageKind>()?),
                None => None,
            },
            force: args.is_present("force"),
            format: "csv",
            infile: str_to_pathbuf(args.value_of("infile")
                                       .ok_or(String::from("missing required infile argument"))?, true)?,
            outdir: str_to_pathbuf(args.value_of("outdir")
                                       .ok_or(String::from("missing required outdir argument"))?, false)?,
            platform: args.values_of("platform")
                          .map_or(Ok(vec![PlatformKind::Android, PlatformKind::iOS]), |platforms| {
                              platforms.map(|platform| platform.parse::<PlatformKind>())
                                       .collect::<Result<Vec<PlatformKind>, String>>()
                          })?
        };

        Ok(settings)
    }
}

/*
 * Converts a string into a PathBuf, ensuring that the path exists and is a file or directory.
 */
fn str_to_pathbuf(path_str: &str, is_file: bool) -> Result<PathBuf, String> {
    let pathbuf = fs::canonicalize(path_str).map_err(|e| format!("\"{}\" {}", path_str, e.description()))?;

    if is_file && !pathbuf.is_file() {
        Err(format!("\"{}\" is not a file", path_str))
    } else if !is_file && !pathbuf.is_dir() {
        Err(format!("\"{}\" is not a directory", path_str))
    } else {
        Ok(pathbuf)
    }
}
