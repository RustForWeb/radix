use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::{
    components::callout_props::{CalloutSize, CalloutSizeProp},
    props::prop_def::{prop_enum, prop_optional_responsive_number_enum, Responsive},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextAs {
    #[default]
    Span,
    Div,
    Label,
    P,
}

impl Display for TextAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAs::Span => "span",
                TextAs::Div => "div",
                TextAs::Label => "label",
                TextAs::P => "p",
            }
        )
    }
}

prop_enum!(TextAsProp, TextAs, None, None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TextSize(u8);

impl Display for TextSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CalloutSize> for TextSize {
    fn from(value: CalloutSize) -> Self {
        match value.0 {
            1 | 2 => Self(2),
            3 => Self(3),
            _ => unreachable!("Callout size is a number between 1 and 3."),
        }
    }
}

impl TryFrom<u8> for TextSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Text size must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_optional_responsive_number_enum!(TextSizeProp, TextSize, Some("rt-r-size"), None);

impl IntoPropValue<TextSizeProp> for CalloutSizeProp {
    fn into_prop_value(self) -> TextSizeProp {
        match self.0 {
            Responsive::Value(size) => TextSizeProp(Some(Responsive::Value(size.into()))),
            Responsive::Values(values) => TextSizeProp(Some(Responsive::Values(
                values
                    .into_iter()
                    .map(|(breakpoint, size)| (breakpoint, size.into()))
                    .collect(),
            ))),
        }
    }
}
