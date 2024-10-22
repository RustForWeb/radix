use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::{
    props::prop_def::{PropDef, PropDefType, PropValue, Responsive},
    ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SectionSize(u8);

impl Default for SectionSize {
    fn default() -> Self {
        Self(3)
    }
}

impl Display for SectionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for SectionSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=4).contains(&value) {
            Err(format!(
                "Section size must be between 1 and 4, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SectionSizeProp(pub SectionSize);

impl IntoPropValue<SectionSizeProp> for u8 {
    fn into_prop_value(self) -> SectionSizeProp {
        SectionSizeProp(self.try_into().unwrap())
    }
}

impl IntoPropValue<SectionSizeProp> for SectionSize {
    fn into_prop_value(self) -> SectionSizeProp {
        SectionSizeProp(self)
    }
}

impl PropDef for SectionSizeProp {
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
        Some(PropValue::String(StringValue::Defined(self.0.to_string())))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SectionDisplay {
    None,
    Initial,
}

impl Display for SectionDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SectionDisplay::None => "none",
                SectionDisplay::Initial => "block",
            }
        )
    }
}

impl From<SectionDisplay> for StringValue {
    fn from(value: SectionDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SectionDisplayProp(pub Option<Responsive<SectionDisplay>>);

impl IntoPropValue<SectionDisplayProp> for SectionDisplay {
    fn into_prop_value(self) -> SectionDisplayProp {
        SectionDisplayProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<SectionDisplayProp> for ResponsiveValues<SectionDisplay> {
    fn into_prop_value(self) -> SectionDisplayProp {
        SectionDisplayProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for SectionDisplayProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-display")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
