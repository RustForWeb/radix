use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AvatarSize(u8);

impl Default for AvatarSize {
    fn default() -> Self {
        Self(3)
    }
}

impl Display for AvatarSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for AvatarSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Avatar size must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(AvatarSizeProp, AvatarSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AvatarVariant {
    Solid,
    #[default]
    Soft,
}

impl Display for AvatarVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AvatarVariant::Solid => "solid",
                AvatarVariant::Soft => "soft",
            }
        )
    }
}

prop_enum!(AvatarVariantProp, AvatarVariant, Some("rt-variant"), None);
