use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HighContrast(pub Option<bool>);

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
}
