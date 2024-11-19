use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardSize(u8);

impl Default for CardSize {
    fn default() -> Self {
        Self(1)
    }
}

impl Display for CardSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for CardSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=5).contains(&value) {
            Err(format!(
                "Card size must be between 1 and 5, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(CardSizeProp, CardSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CardVariant {
    #[default]
    Surface,
    Classic,
    Ghost,
}

impl Display for CardVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardVariant::Surface => "surface",
                CardVariant::Classic => "classic",
                CardVariant::Ghost => "ghost",
            }
        )
    }
}

prop_enum!(CardVariantProp, CardVariant, Some("rt-variant"), None);
