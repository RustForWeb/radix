use std::fmt::{self, Display};

use crate::props::prop_def::{prop_optional_responsive_number_enum_or_string, StringValue};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Gap {
    Defined(u8),
    Arbitrary(String),
}

impl Display for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gap::Defined(value) => write!(f, "{}", value),
            Gap::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for Gap {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=9).contains(&value) {
            Err(format!("Gap must be between 0 and 9, but is {}.", value))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for Gap {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for Gap {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<Gap> for StringValue {
    fn from(value: Gap) -> Self {
        match value {
            Gap::Defined(value) => StringValue::Defined(value.to_string()),
            Gap::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

prop_optional_responsive_number_enum_or_string!(
    GapProp,
    Gap,
    Some("rt-r-gap"),
    Some(&["--gap"]),
    u8
);
prop_optional_responsive_number_enum_or_string!(
    GapXProp,
    Gap,
    Some("rt-r-cg"),
    Some(&["--column-gap"]),
    u8
);
prop_optional_responsive_number_enum_or_string!(
    GapYProp,
    Gap,
    Some("rt-r-rg"),
    Some(&["--row-gap"]),
    u8
);
