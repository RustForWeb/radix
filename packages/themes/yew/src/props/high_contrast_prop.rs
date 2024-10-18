use std::ops::Deref;

use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, ResponsiveValues, StringValue};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HighContrast(pub Option<bool>);

impl Deref for HighContrast {
    type Target = Option<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoPropValue<HighContrast> for bool {
    fn into_prop_value(self) -> HighContrast {
        HighContrast(Some(self))
    }
}

impl PropDef for HighContrast {
    fn r#type(&self) -> PropDefType {
        PropDefType::Bool
    }

    fn class(&self) -> Option<&str> {
        Some("rt-high-contrast")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn string_value(&self) -> Option<StringValue> {
        None
    }

    fn responsive_values(&self) -> Option<ResponsiveValues<StringValue>> {
        None
    }
}
