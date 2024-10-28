use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_optional_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CodeSize(u8);

impl Display for CodeSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for CodeSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Code size must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_optional_responsive_number_enum!(CodeSizeProp, CodeSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CodeVariant {
    Solid,
    #[default]
    Soft,
    Outline,
    Ghost,
}

impl Display for CodeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CodeVariant::Solid => "solid",
                CodeVariant::Soft => "soft",
                CodeVariant::Outline => "outline",
                CodeVariant::Ghost => "ghost",
            }
        )
    }
}

prop_enum!(CodeVariantProp, CodeVariant, Some("rt-variant"), None);
