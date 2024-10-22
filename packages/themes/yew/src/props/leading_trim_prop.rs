use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LeadingTrim {
    Normal,
    Start,
    End,
    Both,
}

impl Display for LeadingTrim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LeadingTrim::Normal => "normal",
                LeadingTrim::Start => "start",
                LeadingTrim::End => "end",
                LeadingTrim::Both => "both",
            }
        )
    }
}

impl From<LeadingTrim> for StringValue {
    fn from(value: LeadingTrim) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LeadingTrimProp(pub Option<Responsive<LeadingTrim>>);

impl IntoPropValue<LeadingTrimProp> for LeadingTrim {
    fn into_prop_value(self) -> LeadingTrimProp {
        LeadingTrimProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<LeadingTrimProp> for ResponsiveValues<LeadingTrim> {
    fn into_prop_value(self) -> LeadingTrimProp {
        LeadingTrimProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for LeadingTrimProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-lt")
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
