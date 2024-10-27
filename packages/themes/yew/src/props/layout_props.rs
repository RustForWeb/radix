use std::fmt::{self, Display};

use crate::props::prop_def::{
    prop_optional_arbitary_responsive_string, prop_optional_responsive_enum,
    prop_optional_responsive_number_enum_or_string, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::Static => "static",
                Position::Relative => "relative",
                Position::Absolute => "absolute",
                Position::Fixed => "fixed",
                Position::Sticky => "sticky",
            }
        )
    }
}

impl From<Position> for StringValue {
    fn from(value: Position) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(PositionProp, Position, Some("rt-r-position"), None);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PositionEdge {
    Defined(i8),
    Arbitrary(String),
}

impl Display for PositionEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionEdge::Defined(value) => write!(f, "{}", value),
            PositionEdge::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<i8> for PositionEdge {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if !(-9..=9).contains(&value) {
            Err(format!(
                "Position edge must be between -9 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for PositionEdge {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for PositionEdge {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<PositionEdge> for StringValue {
    fn from(value: PositionEdge) -> Self {
        match value {
            PositionEdge::Defined(value) => StringValue::Defined(value.to_string()),
            PositionEdge::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

prop_optional_responsive_number_enum_or_string!(
    InsetProp,
    PositionEdge,
    Some("rt-r-inset"),
    Some(&["--inset"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    TopProp,
    PositionEdge,
    Some("rt-r-top"),
    Some(&["--top"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    RightProp,
    PositionEdge,
    Some("rt-r-right"),
    Some(&["--right"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    BottomProp,
    PositionEdge,
    Some("rt-r-bottom"),
    Some(&["--bottom"]),
    i8
);
prop_optional_responsive_number_enum_or_string!(
    LeftProp,
    PositionEdge,
    Some("rt-r-left"),
    Some(&["--left"]),
    i8
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

impl Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Overflow::Visible => "visible",
                Overflow::Hidden => "hidden",
                Overflow::Clip => "clip",
                Overflow::Scroll => "scroll",
                Overflow::Auto => "auto",
            }
        )
    }
}

impl From<Overflow> for StringValue {
    fn from(value: Overflow) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(OverflowProp, Overflow, Some("rt-r-overflow"), None);
prop_optional_responsive_enum!(OverflowXProp, Overflow, Some("rt-r-ox"), None);
prop_optional_responsive_enum!(OverflowYProp, Overflow, Some("rt-r-oy"), None);

prop_optional_arbitary_responsive_string!(FlexBasisProp, Some("rt-r-fb"), Some(&["--flex-basis"]));

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FlexShrink {
    Defined(u8),
    Arbitrary(String),
}

impl Display for FlexShrink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexShrink::Defined(value) => write!(f, "{}", value),
            FlexShrink::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for FlexShrink {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=1).contains(&value) {
            Err(format!("Flex shrink must be 0 or 1, but is {}.", value))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for FlexShrink {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for FlexShrink {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<FlexShrink> for StringValue {
    fn from(value: FlexShrink) -> Self {
        match value {
            FlexShrink::Defined(value) => StringValue::Defined(value.to_string()),
            FlexShrink::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

prop_optional_responsive_number_enum_or_string!(
    FlexShrinkProp,
    FlexShrink,
    Some("rt-r-fs"),
    Some(&["--flex-shrink"]),
    u8
);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FlexGrow {
    Defined(u8),
    Arbitrary(String),
}

impl Display for FlexGrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexGrow::Defined(value) => write!(f, "{}", value),
            FlexGrow::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for FlexGrow {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=1).contains(&value) {
            Err(format!("Flex grow must be 0 or 1, but is {}.", value))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for FlexGrow {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for FlexGrow {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<FlexGrow> for StringValue {
    fn from(value: FlexGrow) -> Self {
        match value {
            FlexGrow::Defined(value) => StringValue::Defined(value.to_string()),
            FlexGrow::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

prop_optional_responsive_number_enum_or_string!(
    FlexGrowProp,
    FlexGrow,
    Some("rt-r-fg"),
    Some(&["--flex-grow"]),
    u8
);

prop_optional_arbitary_responsive_string!(GridAreaProp, Some("rt-r-ga"), Some(&["--grid-area"]));

prop_optional_arbitary_responsive_string!(
    GridColumnProp,
    Some("rt-r-gc"),
    Some(&["--grid-column"])
);
prop_optional_arbitary_responsive_string!(
    GridColumnStartProp,
    Some("rt-r-gcs"),
    Some(&["--grid-column-start"])
);
prop_optional_arbitary_responsive_string!(
    GridColumnEndProp,
    Some("rt-r-gce"),
    Some(&["--grid-column-end"])
);

prop_optional_arbitary_responsive_string!(GridRowProp, Some("rt-r-gr"), Some(&["--grid-row"]));
prop_optional_arbitary_responsive_string!(
    GridRowStartProp,
    Some("rt-r-grs"),
    Some(&["--grid-row-start"])
);
prop_optional_arbitary_responsive_string!(
    GridRowEndProp,
    Some("rt-r-gre"),
    Some(&["--grid-row-end"])
);
