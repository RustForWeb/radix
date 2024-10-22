use std::ops::Deref;

use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, PropValue};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TruncateProp(pub Option<bool>);

impl Deref for TruncateProp {
    type Target = Option<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoPropValue<TruncateProp> for bool {
    fn into_prop_value(self) -> TruncateProp {
        TruncateProp(Some(self))
    }
}

impl PropDef for TruncateProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Bool
    }

    fn class(&self) -> Option<&str> {
        Some("rt-truncate")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.map(PropValue::Bool)
    }
}
