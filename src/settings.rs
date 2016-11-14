use platform::PlatformKind;
use language::LanguageKind;

extern crate clap;

#[derive(Debug)]
pub struct Settings<'a> {
    default_language: Option<LanguageKind>,
    directory: &'a str,
    force: bool,
    format: &'a str,
    input: &'a str,
    platform: Vec<PlatformKind>,
}

impl<'a> Settings<'a> {
    pub fn from_clap(args: &'a clap::ArgMatches) -> Result<Settings<'a>, String> {
        Ok(Settings {
            default_language: match args.value_of("default_language") {
                Some(language_code) => Some(language_code.parse::<LanguageKind>()?),
                None => None,
            },
            directory: args.value_of("outdir").ok_or(String::from("missing output directory"))?,
            force: args.is_present("force"),
            format: "csv",
            input: args.value_of("infile").ok_or(String::from("missing input file"))?,
            platform: arg_to_platforms(args.values_of("platform"))?,
        })
    }
}

fn arg_to_platforms(platforms_option: Option<clap::Values>) -> Result<Vec<PlatformKind>, String> {
    match platforms_option {
        Some(platform_values) => {
            let mut platforms: Vec<PlatformKind> = vec![];

            for platform in platform_values {
                platforms.push(platform.parse::<PlatformKind>()?);
            }

            platforms.sort();
            Ok(platforms)
        },
        None => Ok(vec![PlatformKind::Android, PlatformKind::iOS])
    }
}

