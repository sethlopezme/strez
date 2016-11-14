use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlatformKind {
    Android,
    iOS
}

impl FromStr for PlatformKind {
    type Err = String;
    fn from_str(platform: &str) -> Result<PlatformKind, String> {
        match platform.to_lowercase().as_str() {
            "android" => Ok(PlatformKind::Android),
            "ios" => Ok(PlatformKind::iOS),
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
        assert_eq!("ios".parse::<PlatformKind>(), Ok(PlatformKind::iOS));
        assert!("unknown".parse::<PlatformKind>().is_err());
    }

    #[test]
    fn uppercase_str_to_platformkind() {
        assert_eq!("ANDROID".parse::<PlatformKind>(), Ok(PlatformKind::Android));
    }
}