use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BaseRadioSize(pub(crate) u8);

impl Default for BaseRadioSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for BaseRadioSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for BaseRadioSize {
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

prop_responsive_number_enum!(BaseRadioSizeProp, BaseRadioSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BaseRadioVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

impl Display for BaseRadioVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BaseRadioVariant::Classic => "classic",
                BaseRadioVariant::Surface => "surface",
                BaseRadioVariant::Soft => "soft",
            }
        )
    }
}

prop_enum!(
    BaseRadioVariantProp,
    BaseRadioVariant,
    Some("rt-variant"),
    None
);
