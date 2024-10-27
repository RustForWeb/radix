use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_optional_enum, StringValue};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FlexAs {
    #[default]
    Div,
    Span,
}

impl Display for FlexAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexAs::Div => "div",
                FlexAs::Span => "span",
            }
        )
    }
}

impl From<FlexAs> for StringValue {
    fn from(value: FlexAs) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_enum!(FlexAsProp, FlexAs, None, None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexDisplay {
    None,
    InlineFlex,
    Flex,
}

impl Display for FlexDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexDisplay::None => "none",
                FlexDisplay::InlineFlex => "inline-flex",
                FlexDisplay::Flex => "flex",
            }
        )
    }
}

impl From<FlexDisplay> for StringValue {
    fn from(value: FlexDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_enum!(FlexDisplayProp, FlexDisplay, Some("rt-r-display"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexDirection::Row => "row",
                FlexDirection::Column => "column",
                FlexDirection::RowReverse => "row-reverse",
                FlexDirection::ColumnReverse => "column-reverse",
            }
        )
    }
}

impl From<FlexDirection> for StringValue {
    fn from(value: FlexDirection) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_enum!(FlexDirectionProp, FlexDirection, Some("rt-r-fd"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexAlign {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl Display for FlexAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexAlign::Start => "start",
                FlexAlign::Center => "center",
                FlexAlign::End => "end",
                FlexAlign::Baseline => "baseline",
                FlexAlign::Stretch => "strecht",
            }
        )
    }
}

impl From<FlexAlign> for StringValue {
    fn from(value: FlexAlign) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_enum!(FlexAlignProp, FlexAlign, Some("rt-r-ai"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexJustify {
    Start,
    Center,
    End,
    Between,
}

impl Display for FlexJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexJustify::Start => "start",
                FlexJustify::Center => "center",
                FlexJustify::End => "end",
                FlexJustify::Between => "space-between",
            }
        )
    }
}

impl From<FlexJustify> for StringValue {
    fn from(value: FlexJustify) -> Self {
        StringValue::Defined(value.to_string())
    }
}
prop_optional_enum!(FlexJustifyProp, FlexJustify, Some("rt-r-jc"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

impl Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexWrap::Nowrap => "nowrap",
                FlexWrap::Wrap => "wrap",
                FlexWrap::WrapReverse => "wrap-reverse",
            }
        )
    }
}

impl From<FlexWrap> for StringValue {
    fn from(value: FlexWrap) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_enum!(FlexWrapProp, FlexWrap, Some("rt-r-fw"), None);
