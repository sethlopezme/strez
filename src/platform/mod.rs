use std::fmt;
use std::str::FromStr;

use Result;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlatformKind {
    Android,
    Ios
}

impl fmt::Display for PlatformKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let platform_str = match *self {
            PlatformKind::Android => "Android",
            PlatformKind::Ios => "iOS",
        };

        write!(f, "{}", platform_str)
    }
}

impl FromStr for PlatformKind {
    type Err = String;
    fn from_str(platform: &str) -> Result<PlatformKind> {
        match platform.to_lowercase().as_str() {
            "android" => Ok(PlatformKind::Android),
            "ios" => Ok(PlatformKind::Ios),
            _ => Err(format!("unknown platform \"{}\"", platform)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_platformkind() {
        assert_eq!("android".parse::<PlatformKind>(), Ok(PlatformKind::Android));
        assert_eq!("ios".parse::<PlatformKind>(), Ok(PlatformKind::Ios));
        assert!("unknown".parse::<PlatformKind>().is_err());
    }

    #[test]
    fn uppercase_str_to_platformkind() {
        assert_eq!("ANDROID".parse::<PlatformKind>(), Ok(PlatformKind::Android));
    }
}