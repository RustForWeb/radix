use std::fmt::{self, Display};

use crate::props::prop_def::{
    prop_enum, prop_optional_responsive_enum, prop_responsive_number_enum,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TableSize(u8);

impl Default for TableSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for TableSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for TableSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Table size must be between 1 and 3, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(TableSizeProp, TableSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TableVariant {
    Surface,
    #[default]
    Ghost,
}

impl Display for TableVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TableVariant::Surface => "surface",
                TableVariant::Ghost => "ghost",
            }
        )
    }
}

prop_enum!(TableVariantProp, TableVariant, Some("rt-variant"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TableLayout {
    Auto,
    Fixed,
}

impl Display for TableLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TableLayout::Auto => "auto",
                TableLayout::Fixed => "fixed",
            }
        )
    }
}

prop_optional_responsive_enum!(TableLayoutProp, TableLayout, Some("rt-r-tl"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TableRowAlign {
    Start,
    Center,
    End,
    Baseline,
}

impl Display for TableRowAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TableRowAlign::Start => "top",
                TableRowAlign::Center => "middle",
                TableRowAlign::End => "bottom",
                TableRowAlign::Baseline => "baseline",
            }
        )
    }
}

prop_optional_responsive_enum!(TableRowAlignProp, TableRowAlign, Some("rt-r-va"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TableCellJustify {
    Start,
    Center,
    End,
}

impl Display for TableCellJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TableCellJustify::Start => "left",
                TableCellJustify::Center => "middle",
                TableCellJustify::End => "right",
            }
        )
    }
}

prop_optional_responsive_enum!(
    TableCellJustifyProp,
    TableCellJustify,
    Some("rt-r-ta"),
    None
);
