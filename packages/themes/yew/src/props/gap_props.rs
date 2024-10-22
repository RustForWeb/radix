use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GapProp(pub Option<Responsive<Gap>>);

impl IntoPropValue<GapProp> for Gap {
    fn into_prop_value(self) -> GapProp {
        GapProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GapProp> for u8 {
    fn into_prop_value(self) -> GapProp {
        GapProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<GapProp> for &str {
    fn into_prop_value(self) -> GapProp {
        GapProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GapProp> for String {
    fn into_prop_value(self) -> GapProp {
        GapProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GapProp> for ResponsiveValues<Gap> {
    fn into_prop_value(self) -> GapProp {
        GapProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GapProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gap")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--gap"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GapXProp(pub Option<Responsive<Gap>>);

impl IntoPropValue<GapXProp> for Gap {
    fn into_prop_value(self) -> GapXProp {
        GapXProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GapXProp> for u8 {
    fn into_prop_value(self) -> GapXProp {
        GapXProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<GapXProp> for &str {
    fn into_prop_value(self) -> GapXProp {
        GapXProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GapXProp> for String {
    fn into_prop_value(self) -> GapXProp {
        GapXProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GapXProp> for ResponsiveValues<Gap> {
    fn into_prop_value(self) -> GapXProp {
        GapXProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GapXProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-cg")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--column-gap"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GapYProp(pub Option<Responsive<Gap>>);

impl IntoPropValue<GapYProp> for Gap {
    fn into_prop_value(self) -> GapYProp {
        GapYProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GapYProp> for u8 {
    fn into_prop_value(self) -> GapYProp {
        GapYProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<GapYProp> for &str {
    fn into_prop_value(self) -> GapYProp {
        GapYProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GapYProp> for String {
    fn into_prop_value(self) -> GapYProp {
        GapYProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GapYProp> for ResponsiveValues<Gap> {
    fn into_prop_value(self) -> GapYProp {
        GapYProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GapYProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-rg")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--row-gap"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
