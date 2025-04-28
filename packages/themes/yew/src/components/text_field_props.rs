use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_optional_enum, prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TextFieldSize(u8);

impl Default for TextFieldSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for TextFieldSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for TextFieldSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Text field size must be between 1 and 3, but is {value}."
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(TextFieldSizeProp, TextFieldSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TextFieldVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

impl Display for TextFieldVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextFieldVariant::Classic => "classic",
                TextFieldVariant::Surface => "surface",
                TextFieldVariant::Soft => "soft",
            }
        )
    }
}

prop_enum!(
    TextFieldVariantProp,
    TextFieldVariant,
    Some("rt-variant"),
    None
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextFieldSlotSide {
    Left,
    Right,
}

impl Display for TextFieldSlotSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextFieldSlotSide::Left => "left",
                TextFieldSlotSide::Right => "right",
            }
        )
    }
}

prop_optional_enum!(TextFieldSlotSideProp, TextFieldSlotSide, None, None);
