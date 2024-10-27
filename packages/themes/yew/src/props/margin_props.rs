use std::fmt::{self, Display};

use crate::props::prop_def::{prop_optional_responsive_number_enum_or_string, StringValue};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Margin {
    Defined(i8),
    Arbitrary(String),
}

impl Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Margin::Defined(value) => write!(f, "{}", value),
            Margin::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<i8> for Margin {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if !(-9..=9).contains(&value) {
            Err(format!(
                "Margin must be between -9 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for Margin {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for Margin {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<Margin> for StringValue {
    fn from(value: Margin) -> Self {
        match value {
            Margin::Defined(value) => StringValue::Defined(value.to_string()),
            Margin::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

prop_optional_responsive_number_enum_or_string!(MProp, Margin, Some("rt-r-m"), Some(&["--m"]), i8);
prop_optional_responsive_number_enum_or_string!(
    MxProp,
    Margin,
    Some("rt-r-mx"),
    Some(&["--ml", "--mr"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    MyProp,
    Margin,
    Some("rt-r-my"),
    Some(&["--mt", "--mb"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    MtProp,
    Margin,
    Some("rt-r-mt"),
    Some(&["--mt"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    MrProp,
    Margin,
    Some("rt-r-mr"),
    Some(&["--mr"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    MbProp,
    Margin,
    Some("rt-r-mb"),
    Some(&["--mb"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    MlProp,
    Margin,
    Some("rt-r-ml"),
    Some(&["--ml"]),
    i8
);
