use std::fmt::{self, Display};

use yew::html::IntoPropValue;

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RadiusProp(pub Option<Radius>);

impl IntoPropValue<RadiusProp> for Radius {
    fn into_prop_value(self) -> RadiusProp {
        RadiusProp(Some(self))
    }
}

impl PropDef for RadiusProp {
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
        self.0
            .map(|value| PropValue::String(StringValue::Defined(value.to_string())))
    }
}
