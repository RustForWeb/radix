use std::fmt::{self, Display};

use crate::props::prop_def::{prop_optional_responsive_enum, StringValue};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Weight {
    Light,
    Regular,
    Medium,
    Bold,
}

impl Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Weight::Light => "light",
                Weight::Regular => "regular",
                Weight::Medium => "medium",
                Weight::Bold => "bold",
            }
        )
    }
}

impl From<Weight> for StringValue {
    fn from(value: Weight) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(WeightProp, Weight, Some("rt-r-weight"), None);
