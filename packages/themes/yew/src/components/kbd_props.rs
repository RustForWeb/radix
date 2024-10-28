use std::fmt::{self, Display};

use crate::props::prop_def::prop_optional_responsive_number_enum;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KbdSize(u8);

impl Display for KbdSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for KbdSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Kbd size must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_optional_responsive_number_enum!(KbdSizeProp, KbdSize, Some("rt-r-size"), None);
