use std::fmt::{self, Display};

use crate::{prop_enum, props::prop_def::prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SelectSize(u8);

impl Default for SelectSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for SelectSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for SelectSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Select size must be between 1 and 3, but is {value}."
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(SelectSizeProp, SelectSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SelectTriggerVariant {
    Classic,
    #[default]
    Surface,
    Soft,
    Ghost,
}

impl Display for SelectTriggerVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectTriggerVariant::Classic => "classic",
                SelectTriggerVariant::Surface => "surface",
                SelectTriggerVariant::Soft => "soft",
                SelectTriggerVariant::Ghost => "ghost",
            }
        )
    }
}

prop_enum!(
    SelectTriggerVariantProp,
    SelectTriggerVariant,
    Some("rt-variant"),
    None
);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SelectContentVariant {
    #[default]
    Solid,
    Soft,
}

impl Display for SelectContentVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectContentVariant::Solid => "solid",
                SelectContentVariant::Soft => "soft",
            }
        )
    }
}

prop_enum!(
    SelectContentVariantProp,
    SelectContentVariant,
    Some("rt-variant"),
    None
);
