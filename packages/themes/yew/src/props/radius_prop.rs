use std::fmt::{self, Display};

use crate::props::prop_def::{PropDef, PropDefType, ResponsiveValues, StringValue};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Radius {
    None,
    Small,
    Medium,
    Large,
    Full,
}

impl Display for Radius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Radius::None => "none",
                Radius::Small => "small",
                Radius::Medium => "medium",
                Radius::Large => "large",
                Radius::Full => "full",
            }
        )
    }
}

impl PropDef for Option<Radius> {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        None
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn string_value(&self) -> Option<StringValue> {
        self.map(|value| StringValue::Defined(value.to_string()))
    }

    fn responsive_values(&self) -> Option<ResponsiveValues<StringValue>> {
        None
    }
}
