use std::fmt::{self, Display};

use crate::props::prop_def::{prop_bool, prop_enum, prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BaseButtonSize(pub(crate) u8);

impl Default for BaseButtonSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for BaseButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for BaseButtonSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=4).contains(&value) {
            Err(format!(
                "Select size must be between 1 and 4, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(BaseButtonSizeProp, BaseButtonSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BaseButtonVariant {
    Classic,
    #[default]
    Solid,
    Soft,
    Surface,
    Outline,
    Ghost,
}

impl Display for BaseButtonVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BaseButtonVariant::Classic => "classic",
                BaseButtonVariant::Solid => "solid",
                BaseButtonVariant::Soft => "soft",
                BaseButtonVariant::Surface => "surface",
                BaseButtonVariant::Outline => "outline",
                BaseButtonVariant::Ghost => "ghost",
            }
        )
    }
}

prop_enum!(
    BaseButtonVariantProp,
    BaseButtonVariant,
    Some("rt-variant"),
    None
);

prop_bool!(BaseButtonLoadingProp, Some("rt-loading"), None, false);
