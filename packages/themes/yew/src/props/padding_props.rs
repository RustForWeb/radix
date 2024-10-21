use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PProp(pub Option<Responsive<Padding>>);

impl IntoPropValue<PProp> for Padding {
    fn into_prop_value(self) -> PProp {
        PProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PProp> for i8 {
    fn into_prop_value(self) -> PProp {
        PProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<PProp> for &str {
    fn into_prop_value(self) -> PProp {
        PProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PProp> for String {
    fn into_prop_value(self) -> PProp {
        PProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PProp> for ResponsiveValues<Padding> {
    fn into_prop_value(self) -> PProp {
        PProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-p")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--p"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PxProp(pub Option<Responsive<Padding>>);

impl IntoPropValue<PxProp> for Padding {
    fn into_prop_value(self) -> PxProp {
        PxProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PxProp> for i8 {
    fn into_prop_value(self) -> PxProp {
        PxProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<PxProp> for &str {
    fn into_prop_value(self) -> PxProp {
        PxProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PxProp> for String {
    fn into_prop_value(self) -> PxProp {
        PxProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PxProp> for ResponsiveValues<Padding> {
    fn into_prop_value(self) -> PxProp {
        PxProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PxProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-px")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pl", "--pr"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PyProp(pub Option<Responsive<Padding>>);

impl IntoPropValue<PyProp> for Padding {
    fn into_prop_value(self) -> PyProp {
        PyProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PyProp> for i8 {
    fn into_prop_value(self) -> PyProp {
        PyProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<PyProp> for &str {
    fn into_prop_value(self) -> PyProp {
        PyProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PyProp> for String {
    fn into_prop_value(self) -> PyProp {
        PyProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PyProp> for ResponsiveValues<Padding> {
    fn into_prop_value(self) -> PyProp {
        PyProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PyProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-py")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pt", "--pb"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PtProp(pub Option<Responsive<Padding>>);

impl IntoPropValue<PtProp> for Padding {
    fn into_prop_value(self) -> PtProp {
        PtProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PtProp> for i8 {
    fn into_prop_value(self) -> PtProp {
        PtProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<PtProp> for &str {
    fn into_prop_value(self) -> PtProp {
        PtProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PtProp> for String {
    fn into_prop_value(self) -> PtProp {
        PtProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PtProp> for ResponsiveValues<Padding> {
    fn into_prop_value(self) -> PtProp {
        PtProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PtProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pt")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pt"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PrProp(pub Option<Responsive<Padding>>);

impl IntoPropValue<PrProp> for Padding {
    fn into_prop_value(self) -> PrProp {
        PrProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PrProp> for i8 {
    fn into_prop_value(self) -> PrProp {
        PrProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<PrProp> for &str {
    fn into_prop_value(self) -> PrProp {
        PrProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PrProp> for String {
    fn into_prop_value(self) -> PrProp {
        PrProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PrProp> for ResponsiveValues<Padding> {
    fn into_prop_value(self) -> PrProp {
        PrProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PrProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pr")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pr"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PbProp(pub Option<Responsive<Padding>>);

impl IntoPropValue<PbProp> for Padding {
    fn into_prop_value(self) -> PbProp {
        PbProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PbProp> for i8 {
    fn into_prop_value(self) -> PbProp {
        PbProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<PbProp> for &str {
    fn into_prop_value(self) -> PbProp {
        PbProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PbProp> for String {
    fn into_prop_value(self) -> PbProp {
        PbProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PbProp> for ResponsiveValues<Padding> {
    fn into_prop_value(self) -> PbProp {
        PbProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PbProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pb")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pb"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlProp(pub Option<Responsive<Padding>>);

impl IntoPropValue<PlProp> for Padding {
    fn into_prop_value(self) -> PlProp {
        PlProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PlProp> for i8 {
    fn into_prop_value(self) -> PlProp {
        PlProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<PlProp> for &str {
    fn into_prop_value(self) -> PlProp {
        PlProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PlProp> for String {
    fn into_prop_value(self) -> PlProp {
        PlProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<PlProp> for ResponsiveValues<Padding> {
    fn into_prop_value(self) -> PlProp {
        PlProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PlProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pl")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pl"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
