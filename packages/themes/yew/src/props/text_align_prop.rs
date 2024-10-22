use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextWrap {
    Wrap,
    Nowrap,
    Pretty,
    Balance,
}

impl Display for TextWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextWrap::Wrap => "wrap",
                TextWrap::Nowrap => "nowrap",
                TextWrap::Pretty => "pretty",
                TextWrap::Balance => "balance",
            }
        )
    }
}

impl From<TextWrap> for StringValue {
    fn from(value: TextWrap) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextWrapProp(pub Option<Responsive<TextWrap>>);

impl IntoPropValue<TextWrapProp> for TextWrap {
    fn into_prop_value(self) -> TextWrapProp {
        TextWrapProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<TextWrapProp> for ResponsiveValues<TextWrap> {
    fn into_prop_value(self) -> TextWrapProp {
        TextWrapProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for TextWrapProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-ta")
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
