use std::fmt::{self, Display};

use crate::props::prop_def::{
    prop_enum, prop_optional_responsive_enum, prop_responsive_number_enum, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TextAreaSize(u8);

impl Default for TextAreaSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for TextAreaSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for TextAreaSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Text field size must be between 1 and 3, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(TextAreaSizeProp, TextAreaSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextAreaVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

impl Display for TextAreaVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAreaVariant::Classic => "classic",
                TextAreaVariant::Surface => "surface",
                TextAreaVariant::Soft => "soft",
            }
        )
    }
}

prop_enum!(
    TextAreaVariantProp,
    TextAreaVariant,
    Some("rt-variant"),
    None
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextAreaResize {
    None,
    Vertical,
    Horizontal,
    Both,
}

impl Display for TextAreaResize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAreaResize::None => "none",
                TextAreaResize::Vertical => "vertical",
                TextAreaResize::Horizontal => "horizontal",
                TextAreaResize::Both => "both",
            }
        )
    }
}

impl From<TextAreaResize> for StringValue {
    fn from(value: TextAreaResize) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(
    TextAreaResizeProp,
    TextAreaResize,
    Some("rt-r-resize"),
    None
);
