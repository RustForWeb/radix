use std::fmt::{self, Display};

use crate::props::prop_def::{
    prop_optional_responsive_enum, prop_responsive_enum, prop_responsive_number_enum,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum DataListOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Display for DataListOrientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataListOrientation::Horizontal => "horizontal",
                DataListOrientation::Vertical => "vertical",
            }
        )
    }
}

prop_responsive_enum!(
    DataListOrientationProp,
    DataListOrientation,
    Some("rt-r-orientation"),
    None
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DataListSize(u8);

impl Default for DataListSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for DataListSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for DataListSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Data list size must be between 1 and 3, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(DataListSizeProp, DataListSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataListLeadingTrim {
    Normal,
    Start,
    End,
    Both,
}

impl Display for DataListLeadingTrim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataListLeadingTrim::Normal => "normal",
                DataListLeadingTrim::Start => "start",
                DataListLeadingTrim::End => "end",
                DataListLeadingTrim::Both => "both",
            }
        )
    }
}

prop_optional_responsive_enum!(
    DataListLeadingTrimProp,
    DataListLeadingTrim,
    Some("rt-r-trim"),
    None
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataListItemAlign {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl Display for DataListItemAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataListItemAlign::Start => "start",
                DataListItemAlign::Center => "center",
                DataListItemAlign::End => "end",
                DataListItemAlign::Baseline => "baseline",
                DataListItemAlign::Stretch => "stretch",
            }
        )
    }
}

prop_optional_responsive_enum!(
    DataListItemAlignProp,
    DataListItemAlign,
    Some("rt-r-ai"),
    None
);
