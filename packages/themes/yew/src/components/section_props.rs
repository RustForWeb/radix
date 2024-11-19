use std::fmt::{self, Display};

use crate::{prop_optional_responsive_enum, props::prop_def::prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SectionSize(u8);

impl Default for SectionSize {
    fn default() -> Self {
        Self(3)
    }
}

impl Display for SectionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for SectionSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=4).contains(&value) {
            Err(format!(
                "Heading size must be between 1 and 4, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(SectionSizeProp, SectionSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SectionDisplay {
    None,
    Initial,
}

impl Display for SectionDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SectionDisplay::None => "none",
                SectionDisplay::Initial => "block",
            }
        )
    }
}

prop_optional_responsive_enum!(
    SectionDisplayProp,
    SectionDisplay,
    Some("rt-r-display"),
    None
);
