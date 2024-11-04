use std::fmt::{self, Display};

use radix_yew_separator::Orientation;

use crate::props::prop_def::{prop_bool, prop_responsive_enum, prop_responsive_number_enum};

pub type SeparatorOrientation = Orientation;

prop_responsive_enum!(
    SeparatorOrientationProp,
    SeparatorOrientation,
    Some("rt-r-orientation"),
    None
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SeparatorSize(u8);

impl Default for SeparatorSize {
    fn default() -> Self {
        Self(1)
    }
}

impl Display for SeparatorSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for SeparatorSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=4).contains(&value) {
            Err(format!(
                "Separator size must be between 1 and 4, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(SeparatorSizeProp, SeparatorSize, Some("rt-r-size"), None);

prop_bool!(SeparatorDecorativeProp, None, None, true);
