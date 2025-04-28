use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_responsive_number_enum};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum HeadingAs {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl Display for HeadingAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HeadingAs::H1 => "h1",
                HeadingAs::H2 => "h2",
                HeadingAs::H3 => "h3",
                HeadingAs::H4 => "h4",
                HeadingAs::H5 => "h5",
                HeadingAs::H6 => "h6",
            }
        )
    }
}

prop_enum!(HeadingAsProp, HeadingAs, None, None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HeadingSize(u8);

impl Default for HeadingSize {
    fn default() -> Self {
        Self(6)
    }
}

impl Display for HeadingSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for HeadingSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Heading size must be between 1 and 9, but is {value}."
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(HeadingSizeProp, HeadingSize, Some("rt-r-size"), None);
