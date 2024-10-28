use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_optional_responsive_number_enum, StringValue};

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

impl From<TextAs> for StringValue {
    fn from(value: TextAs) -> Self {
        StringValue::Defined(value.to_string())
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
