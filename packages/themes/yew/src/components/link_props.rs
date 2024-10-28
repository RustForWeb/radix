use std::fmt::{self, Display};

use crate::props::prop_def::prop_enum;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum LinkUnderline {
    #[default]
    Auto,
    Always,
    Hover,
    None,
}

impl Display for LinkUnderline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LinkUnderline::Auto => "auto",
                LinkUnderline::Always => "always",
                LinkUnderline::Hover => "hover",
                LinkUnderline::None => "none",
            }
        )
    }
}

prop_enum!(LinkUnderlineProp, LinkUnderline, Some("rt-underline"), None);
