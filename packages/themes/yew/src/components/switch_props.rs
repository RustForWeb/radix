use std::fmt::{self, Display};

use crate::props::prop_def::{PropDef, PropDefType, PropValue, StringValue};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SwitchSize {
    S1,
    #[default]
    S2,
    S3,
}

impl Display for SwitchSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SwitchSize::S1 => "1",
                SwitchSize::S2 => "2",
                SwitchSize::S3 => "3",
            }
        )
    }
}

impl PropDef for SwitchSize {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-size")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(self.to_string())))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SwitchVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

impl Display for SwitchVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SwitchVariant::Classic => "classic",
                SwitchVariant::Surface => "surface",
                SwitchVariant::Soft => "soft",
            }
        )
    }
}

impl PropDef for SwitchVariant {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-variant")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(self.to_string())))
    }
}
