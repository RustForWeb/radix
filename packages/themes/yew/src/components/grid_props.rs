use std::fmt::{self, Display};

use crate::{
    prop_enum, prop_optional_arbitary_responsive_string, prop_optional_responsive_enum,
    prop_optional_responsive_number_enum_or_string, props::prop_def::StringValue,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GridAs {
    #[default]
    Div,
    Span,
}

impl Display for GridAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridAs::Div => "div",
                GridAs::Span => "span",
            }
        )
    }
}

impl From<GridAs> for StringValue {
    fn from(value: GridAs) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_enum!(GridAsProp, GridAs, None, None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridDisplay {
    None,
    InlineGrid,
    Grid,
}

impl Display for GridDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridDisplay::None => "none",
                GridDisplay::InlineGrid => "inline-grid",
                GridDisplay::Grid => "grid",
            }
        )
    }
}

impl From<GridDisplay> for StringValue {
    fn from(value: GridDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(GridDisplayProp, GridDisplay, Some("rt-r-display"), None);

prop_optional_arbitary_responsive_string!(
    GridAreasProp,
    Some("rt-r-gta"),
    Some(&["--grid-template-areas"])
);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GridColumns {
    Defined(u8),
    Arbitrary(String),
}

impl Display for GridColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridColumns::Defined(value) => write!(f, "{}", value),
            GridColumns::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for GridColumns {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Grid columns must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for GridColumns {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for GridColumns {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<GridColumns> for StringValue {
    fn from(value: GridColumns) -> Self {
        match value {
            GridColumns::Defined(value) => StringValue::Defined(value.to_string()),
            GridColumns::Arbitrary(value) => {
                if let Ok(number) = value.parse::<u8>() {
                    StringValue::Arbitrary(format!("repeat({number}, minmax(0, 1fr))"))
                } else {
                    StringValue::Arbitrary(value.to_string())
                }
            }
        }
    }
}

prop_optional_responsive_number_enum_or_string!(
    GridColumnsProp,
    GridColumns,
    Some("rt-r-gtc"),
    Some(&["--grid-template-columns"]),
    u8
);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GridRows {
    Defined(u8),
    Arbitrary(String),
}

impl Display for GridRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridRows::Defined(value) => write!(f, "{}", value),
            GridRows::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for GridRows {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Grid rows must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for GridRows {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for GridRows {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<GridRows> for StringValue {
    fn from(value: GridRows) -> Self {
        match value {
            GridRows::Defined(value) => StringValue::Defined(value.to_string()),
            GridRows::Arbitrary(value) => {
                if let Ok(number) = value.parse::<u8>() {
                    StringValue::Arbitrary(format!("repeat({number}, minmax(0, 1fr))"))
                } else {
                    StringValue::Arbitrary(value.to_string())
                }
            }
        }
    }
}

prop_optional_responsive_number_enum_or_string!(
    GridRowsProp,
    GridRows,
    Some("rt-r-gtr"),
    Some(&["--grid-template-rows"]),
    u8
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridFlow {
    Row,
    Column,
    Dense,
    RowDense,
    ColumnDense,
}

impl Display for GridFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridFlow::Row => "row",
                GridFlow::Column => "column",
                GridFlow::Dense => "dense",
                GridFlow::RowDense => "row-dense",
                GridFlow::ColumnDense => "column-dense",
            }
        )
    }
}

impl From<GridFlow> for StringValue {
    fn from(value: GridFlow) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(GridFlowProp, GridFlow, Some("rt-r-gaf"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridAlign {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl Display for GridAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridAlign::Start => "start",
                GridAlign::Center => "center",
                GridAlign::End => "end",
                GridAlign::Baseline => "baseline",
                GridAlign::Stretch => "strecht",
            }
        )
    }
}

impl From<GridAlign> for StringValue {
    fn from(value: GridAlign) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(GridAlignProp, GridAlign, Some("rt-r-ai"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridJustify {
    Start,
    Center,
    End,
    Between,
}

impl Display for GridJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridJustify::Start => "start",
                GridJustify::Center => "center",
                GridJustify::End => "end",
                GridJustify::Between => "space-between",
            }
        )
    }
}

impl From<GridJustify> for StringValue {
    fn from(value: GridJustify) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(GridJustifyProp, GridJustify, Some("rt-r-jc"), None);
