use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Weight {
    Light,
    Regular,
    Medium,
    Bold,
}

impl Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Weight::Light => "light",
                Weight::Regular => "regular",
                Weight::Medium => "medium",
                Weight::Bold => "bold",
            }
        )
    }
}

impl From<Weight> for StringValue {
    fn from(value: Weight) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WeightProp(pub Option<Responsive<Weight>>);

impl IntoPropValue<WeightProp> for Weight {
    fn into_prop_value(self) -> WeightProp {
        WeightProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<WeightProp> for ResponsiveValues<Weight> {
    fn into_prop_value(self) -> WeightProp {
        WeightProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for WeightProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-weight")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
