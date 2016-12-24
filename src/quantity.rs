use std::fmt;
use std::str::FromStr;

use Result;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Quantity {
    Single,
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let quantity_str = match *self {
            Quantity::Single => "single",
            Quantity::Zero => "zero",
            Quantity::One => "one",
            Quantity::Two => "two",
            Quantity::Few => "few",
            Quantity::Many => "many",
            Quantity::Other => "other",
        };

        write!(f, "{}", quantity_str)
    }
}

impl FromStr for Quantity {
    type Err = String;
    fn from_str(quantity: &str) -> Result<Quantity> {
        match quantity.to_lowercase().trim() {
            "" | "single" => Ok(Quantity::Single),
            "zero" => Ok(Quantity::Zero),
            "one" => Ok(Quantity::One),
            "two" => Ok(Quantity::Two),
            "few" => Ok(Quantity::Few),
            "many" => Ok(Quantity::Many),
            "other" => Ok(Quantity::Other),
            _ => Err(format!("unknown quantity \"{}\"", quantity)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_quantity() {
        assert_eq!("".parse::<Quantity>(), Ok(Quantity::Single));
        assert_eq!("single".parse::<Quantity>(), Ok(Quantity::Single));
        assert_eq!("zero".parse::<Quantity>(), Ok(Quantity::Zero));
        assert_eq!("one".parse::<Quantity>(), Ok(Quantity::One));
        assert_eq!("two".parse::<Quantity>(), Ok(Quantity::Two));
        assert_eq!("few".parse::<Quantity>(), Ok(Quantity::Few));
        assert_eq!("many".parse::<Quantity>(), Ok(Quantity::Many));
        assert_eq!("other".parse::<Quantity>(), Ok(Quantity::Other));
    }

    #[test]
    fn str_to_unknown_quantity() {
        assert!("unknown".parse::<Quantity>().is_err());
    }

    fn uppercase_str_to_quantity() {
        assert_eq!("ZERO".parse::<Quantity>(), Ok(Quantity::Zero));
    }
}