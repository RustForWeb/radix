use std::fmt::{self, Display};

use crate::props::prop_def::{prop_optional_responsive_number_enum_or_string, StringValue};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Padding {
    Defined(i8),
    Arbitrary(String),
}

impl Display for Padding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Padding::Defined(value) => write!(f, "{}", value),
            Padding::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<i8> for Padding {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if !(-9..=9).contains(&value) {
            Err(format!(
                "Padding must be between -9 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for Padding {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for Padding {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<Padding> for StringValue {
    fn from(value: Padding) -> Self {
        match value {
            Padding::Defined(value) => StringValue::Defined(value.to_string()),
            Padding::Arbitrary(value) => StringValue::Arbitrary(value),
        }
    }
}

prop_optional_responsive_number_enum_or_string!(PProp, Padding, Some("rt-r-p"), Some(&["--p"]), i8);
prop_optional_responsive_number_enum_or_string!(
    PxProp,
    Padding,
    Some("rt-r-px"),
    Some(&["--pl", "--pr"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    PyProp,
    Padding,
    Some("rt-r-py"),
    Some(&["--pt", "--pb"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    PtProp,
    Padding,
    Some("rt-r-pt"),
    Some(&["--pt"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    PrProp,
    Padding,
    Some("rt-r-pr"),
    Some(&["--pr"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    PbProp,
    Padding,
    Some("rt-r-pb"),
    Some(&["--pb"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    PlProp,
    Padding,
    Some("rt-r-pl"),
    Some(&["--pl"]),
    i8
);
