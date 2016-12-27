use std::collections::HashMap;
use std::fmt;
use std::io;
use std::str::FromStr;

use Result;
use resource::Resource;

mod csv;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Format {
    CSV,
    XLIFF,
}

impl Format {
    pub fn parse_reader<R: io::Read>(&self, reader: R) -> Result<HashMap<String, Resource>> {
        match *self {
            Format::CSV => csv::parse_reader(reader),
            ref f => Err(format!("{} format is not supported right now", f)),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_str = match *self {
            Format::CSV => "CSV",
            Format::XLIFF => "XLIFF",
        };

        write!(f, "{}", format_str)
    }
}

impl FromStr for Format {
    type Err = String;
    fn from_str(format: &str) -> Result<Format> {
        match format.to_lowercase().as_str() {
            "csv" => Ok(Format::CSV),
            "xliff" => Ok(Format::XLIFF),
            _ => Err(format!("unknown format \"{}\"", format)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_format() {
        assert_eq!("csv".parse::<Format>(), Ok(Format::CSV));
        assert_eq!("xliff".parse::<Format>(), Ok(Format::XLIFF));
        assert!("unknown".parse::<Format>().is_err());
    }

    #[test]
    fn uppercase_str_to_format() {
        assert_eq!("CSV".parse::<Format>(), Ok(Format::CSV));
    }
}