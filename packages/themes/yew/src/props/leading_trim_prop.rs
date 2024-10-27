use std::fmt::{self, Display};

use crate::props::prop_def::{prop_optional_responsive_enum, StringValue};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LeadingTrim {
    Normal,
    Start,
    End,
    Both,
}

impl Display for LeadingTrim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LeadingTrim::Normal => "normal",
                LeadingTrim::Start => "start",
                LeadingTrim::End => "end",
                LeadingTrim::Both => "both",
            }
        )
    }
}

impl From<LeadingTrim> for StringValue {
    fn from(value: LeadingTrim) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(LeadingTrimProp, LeadingTrim, Some("rt-r-lt"), None);
