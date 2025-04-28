use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BadgeSize(u8);

impl Default for BadgeSize {
    fn default() -> Self {
        Self(1)
    }
}

impl Display for BadgeSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for BadgeSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Badge size must be between 1 and 3, but is {value}."
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(BadgeSizeProp, BadgeSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BadgeVariant {
    Solid,
    #[default]
    Soft,
    Surface,
    Outline,
}

impl Display for BadgeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BadgeVariant::Solid => "solid",
                BadgeVariant::Soft => "soft",
                BadgeVariant::Surface => "surface",
                BadgeVariant::Outline => "outline",
            }
        )
    }
}

prop_enum!(BadgeVariantProp, BadgeVariant, Some("rt-variant"), None);
