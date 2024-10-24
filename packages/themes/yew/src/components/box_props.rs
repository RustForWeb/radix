use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BoxAs {
    #[default]
    Div,
    Span,
}

impl Display for BoxAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxAs::Div => "div",
                BoxAs::Span => "span",
            }
        )
    }
}

impl From<BoxAs> for StringValue {
    fn from(value: BoxAs) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoxAsProp(pub BoxAs);

impl IntoPropValue<BoxAsProp> for BoxAs {
    fn into_prop_value(self) -> BoxAsProp {
        BoxAsProp(self)
    }
}

impl PropDef for BoxAsProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        None
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(self.0.to_string())))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BoxDisplay {
    None,
    Inline,
    InlineBlock,
    Block,
}

impl Display for BoxDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxDisplay::None => "none",
                BoxDisplay::Inline => "inline",
                BoxDisplay::InlineBlock => "inline-block",
                BoxDisplay::Block => "block",
            }
        )
    }
}

impl From<BoxDisplay> for StringValue {
    fn from(value: BoxDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoxDisplayProp(pub Option<Responsive<BoxDisplay>>);

impl IntoPropValue<BoxDisplayProp> for BoxDisplay {
    fn into_prop_value(self) -> BoxDisplayProp {
        BoxDisplayProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<BoxDisplayProp> for ResponsiveValues<BoxDisplay> {
    fn into_prop_value(self) -> BoxDisplayProp {
        BoxDisplayProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for BoxDisplayProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-display")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
