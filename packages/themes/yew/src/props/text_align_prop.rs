use std::fmt::{self, Display};

use crate::props::prop_def::prop_optional_responsive_enum;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAlign::Left => "left",
                TextAlign::Center => "center",
                TextAlign::Right => "right",
            }
        )
    }
}

prop_optional_responsive_enum!(TextAlignProp, TextAlign, Some("rt-r-ta"), None);
