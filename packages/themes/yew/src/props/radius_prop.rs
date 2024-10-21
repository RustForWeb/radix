use std::fmt::{self, Display};

use crate::props::prop_def::{PropDef, PropDefType, PropValue, StringValue};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Radius {
    None,
    Small,
    #[default]
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

    fn value(&self) -> Option<PropValue> {
        self.map(|value| PropValue::String(StringValue::Defined(value.to_string())))
    }
}
