pub mod csv;

use std::fmt;
use std::io;
use std::str::FromStr;

use Result;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FormatKind {
    CSV,
    XLIFF
}

impl fmt::Display for FormatKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_str = match *self {
            FormatKind::CSV => "CSV",
            FormatKind::XLIFF => "XLIFF",
        };

        write!(f, "{}", format_str)
    }
}

impl FromStr for FormatKind {
    type Err = String;
    fn from_str(format: &str) -> Result<FormatKind> {
        match format.to_lowercase().as_str() {
            "csv" => Ok(FormatKind::CSV),
            "xliff" => Ok(FormatKind::XLIFF),
            _ => Err(format!("unknown format \"{}\"", format)),
        }
    }
}

pub trait Format: Sized {
    fn from_reader<R: io::Read>(r: R) -> Result<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_formatkind() {
        assert_eq!("csv".parse::<FormatKind>(), Ok(FormatKind::CSV));
        assert_eq!("xliff".parse::<FormatKind>(), Ok(FormatKind::XLIFF));
        assert!("unknown".parse::<FormatKind>().is_err());
    }

    #[test]
    fn uppercase_str_to_formatkind() {
        assert_eq!("CSV".parse::<FormatKind>(), Ok(FormatKind::CSV));
    }
}