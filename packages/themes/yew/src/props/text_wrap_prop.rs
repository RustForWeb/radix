use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAlign::Left => "left",
                TextAlign::Center => "center",
                TextAlign::Right => "right",
            }
        )
    }
}

impl From<TextAlign> for StringValue {
    fn from(value: TextAlign) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextAlignProp(pub Option<Responsive<TextAlign>>);

impl IntoPropValue<TextAlignProp> for TextAlign {
    fn into_prop_value(self) -> TextAlignProp {
        TextAlignProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<TextAlignProp> for ResponsiveValues<TextAlign> {
    fn into_prop_value(self) -> TextAlignProp {
        TextAlignProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for TextAlignProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-tw")
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
