use std::fmt::{self, Display};

use crate::props::prop_def::{prop_optional_responsive_enum, StringValue};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextWrap {
    Wrap,
    Nowrap,
    Pretty,
    Balance,
}

impl Display for TextWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextWrap::Wrap => "wrap",
                TextWrap::Nowrap => "nowrap",
                TextWrap::Pretty => "pretty",
                TextWrap::Balance => "balance",
            }
        )
    }
}

impl From<TextWrap> for StringValue {
    fn from(value: TextWrap) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(TextWrapProp, TextWrap, Some("rt-r-tw"), None);
