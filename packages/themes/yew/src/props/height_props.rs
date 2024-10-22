use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, PropValue, Responsive, ResponsiveValues};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeightProp(pub Option<Responsive<String>>);

impl IntoPropValue<HeightProp> for &str {
    fn into_prop_value(self) -> HeightProp {
        HeightProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<HeightProp> for String {
    fn into_prop_value(self) -> HeightProp {
        HeightProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<HeightProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> HeightProp {
        HeightProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for HeightProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-h")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--height"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MinHeightProp(pub Option<Responsive<String>>);

impl IntoPropValue<MinHeightProp> for &str {
    fn into_prop_value(self) -> MinHeightProp {
        MinHeightProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MinHeightProp> for String {
    fn into_prop_value(self) -> MinHeightProp {
        MinHeightProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MinHeightProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MinHeightProp {
        MinHeightProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for MinHeightProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-min-h")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--min-height"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaxHeightProp(pub Option<Responsive<String>>);

impl IntoPropValue<MaxHeightProp> for &str {
    fn into_prop_value(self) -> MaxHeightProp {
        MaxHeightProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<MaxHeightProp> for String {
    fn into_prop_value(self) -> MaxHeightProp {
        MaxHeightProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<MaxHeightProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> MaxHeightProp {
        MaxHeightProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for MaxHeightProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-max-h")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--max-height"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}
