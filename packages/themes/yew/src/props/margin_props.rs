use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MProp(pub Option<Responsive<Margin>>);

impl IntoPropValue<MProp> for Margin {
    fn into_prop_value(self) -> MProp {
        MProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MProp> for i8 {
    fn into_prop_value(self) -> MProp {
        MProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<MProp> for &str {
    fn into_prop_value(self) -> MProp {
        MProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MProp> for String {
    fn into_prop_value(self) -> MProp {
        MProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MProp> for ResponsiveValues<Margin> {
    fn into_prop_value(self) -> MProp {
        MProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<MProp> for ResponsiveValues<i8> {
    fn into_prop_value(self) -> MProp {
        MProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<MProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MProp {
        MProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for MProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-m")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--m"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MxProp(pub Option<Responsive<Margin>>);

impl IntoPropValue<MxProp> for Margin {
    fn into_prop_value(self) -> MxProp {
        MxProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MxProp> for i8 {
    fn into_prop_value(self) -> MxProp {
        MxProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<MxProp> for &str {
    fn into_prop_value(self) -> MxProp {
        MxProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MxProp> for String {
    fn into_prop_value(self) -> MxProp {
        MxProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MxProp> for ResponsiveValues<Margin> {
    fn into_prop_value(self) -> MxProp {
        MxProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<MxProp> for ResponsiveValues<i8> {
    fn into_prop_value(self) -> MxProp {
        MxProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<MxProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MxProp {
        MxProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for MxProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mx")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--ml", "--mr"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MyProp(pub Option<Responsive<Margin>>);

impl IntoPropValue<MyProp> for Margin {
    fn into_prop_value(self) -> MyProp {
        MyProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MyProp> for i8 {
    fn into_prop_value(self) -> MyProp {
        MyProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<MyProp> for &str {
    fn into_prop_value(self) -> MyProp {
        MyProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MyProp> for String {
    fn into_prop_value(self) -> MyProp {
        MyProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MyProp> for ResponsiveValues<Margin> {
    fn into_prop_value(self) -> MyProp {
        MyProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<MyProp> for ResponsiveValues<i8> {
    fn into_prop_value(self) -> MyProp {
        MyProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<MyProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MyProp {
        MyProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for MyProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-my")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mt", "--mb"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MtProp(pub Option<Responsive<Margin>>);

impl IntoPropValue<MtProp> for Margin {
    fn into_prop_value(self) -> MtProp {
        MtProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MtProp> for i8 {
    fn into_prop_value(self) -> MtProp {
        MtProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<MtProp> for &str {
    fn into_prop_value(self) -> MtProp {
        MtProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MtProp> for String {
    fn into_prop_value(self) -> MtProp {
        MtProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MtProp> for ResponsiveValues<Margin> {
    fn into_prop_value(self) -> MtProp {
        MtProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<MtProp> for ResponsiveValues<i8> {
    fn into_prop_value(self) -> MtProp {
        MtProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<MtProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MtProp {
        MtProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for MtProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mt")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mt"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MrProp(pub Option<Responsive<Margin>>);

impl IntoPropValue<MrProp> for Margin {
    fn into_prop_value(self) -> MrProp {
        MrProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MrProp> for i8 {
    fn into_prop_value(self) -> MrProp {
        MrProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<MrProp> for &str {
    fn into_prop_value(self) -> MrProp {
        MrProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MrProp> for String {
    fn into_prop_value(self) -> MrProp {
        MrProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MrProp> for ResponsiveValues<Margin> {
    fn into_prop_value(self) -> MrProp {
        MrProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<MrProp> for ResponsiveValues<i8> {
    fn into_prop_value(self) -> MrProp {
        MrProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<MrProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MrProp {
        MrProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for MrProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mr")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mr"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MbProp(pub Option<Responsive<Margin>>);

impl IntoPropValue<MbProp> for Margin {
    fn into_prop_value(self) -> MbProp {
        MbProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MbProp> for i8 {
    fn into_prop_value(self) -> MbProp {
        MbProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<MbProp> for &str {
    fn into_prop_value(self) -> MbProp {
        MbProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MbProp> for String {
    fn into_prop_value(self) -> MbProp {
        MbProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MbProp> for ResponsiveValues<Margin> {
    fn into_prop_value(self) -> MbProp {
        MbProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<MbProp> for ResponsiveValues<i8> {
    fn into_prop_value(self) -> MbProp {
        MbProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<MbProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MbProp {
        MbProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for MbProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mb")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mb"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MlProp(pub Option<Responsive<Margin>>);

impl IntoPropValue<MlProp> for Margin {
    fn into_prop_value(self) -> MlProp {
        MlProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MlProp> for i8 {
    fn into_prop_value(self) -> MlProp {
        MlProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<MlProp> for &str {
    fn into_prop_value(self) -> MlProp {
        MlProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MlProp> for String {
    fn into_prop_value(self) -> MlProp {
        MlProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MlProp> for ResponsiveValues<Margin> {
    fn into_prop_value(self) -> MlProp {
        MlProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<MlProp> for ResponsiveValues<i8> {
    fn into_prop_value(self) -> MlProp {
        MlProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<MlProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MlProp {
        MlProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for MlProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-ml")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--ml"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
