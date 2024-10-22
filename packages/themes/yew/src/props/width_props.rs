use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, PropValue, Responsive, ResponsiveValues};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WidthProp(pub Option<Responsive<String>>);

impl IntoPropValue<WidthProp> for &str {
    fn into_prop_value(self) -> WidthProp {
        WidthProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<WidthProp> for String {
    fn into_prop_value(self) -> WidthProp {
        WidthProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<WidthProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> WidthProp {
        WidthProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for WidthProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-w")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--width"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MinWidthProp(pub Option<Responsive<String>>);

impl IntoPropValue<MinWidthProp> for &str {
    fn into_prop_value(self) -> MinWidthProp {
        MinWidthProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MinWidthProp> for String {
    fn into_prop_value(self) -> MinWidthProp {
        MinWidthProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MinWidthProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MinWidthProp {
        MinWidthProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for MinWidthProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-min-w")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--min-width"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaxWidthProp(pub Option<Responsive<String>>);

impl IntoPropValue<MaxWidthProp> for &str {
    fn into_prop_value(self) -> MaxWidthProp {
        MaxWidthProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MaxWidthProp> for String {
    fn into_prop_value(self) -> MaxWidthProp {
        MaxWidthProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MaxWidthProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MaxWidthProp {
        MaxWidthProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for MaxWidthProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-max-w")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--max-width"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}
