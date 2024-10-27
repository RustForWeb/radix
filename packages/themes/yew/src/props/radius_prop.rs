use std::fmt::{self, Display};

use crate::props::prop_def::prop_optional_enum;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Radius {
    None,
    Small,
    #[default]
    Medium,
    Large,
    Full,
}

impl Display for Radius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Radius::None => "none",
                Radius::Small => "small",
                Radius::Medium => "medium",
                Radius::Large => "large",
                Radius::Full => "full",
            }
        )
    }
}

prop_optional_enum!(RadiusProp, Radius, None, None);
