use std::fmt::{self, Display};

use crate::props::prop_def::{prop_optional_responsive_enum, prop_responsive_enum};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum InsetSide {
    #[default]
    All,
    X,
    Y,
    Top,
    Bottom,
    Left,
    Right,
}

impl Display for InsetSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InsetSide::All => "all",
                InsetSide::X => "x",
                InsetSide::Y => "y",
                InsetSide::Top => "top",
                InsetSide::Bottom => "bottom",
                InsetSide::Left => "left",
                InsetSide::Right => "right",
            }
        )
    }
}

prop_responsive_enum!(InsetSideProp, InsetSide, Some("rt-r-side"), None);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum InsetClip {
    #[default]
    BorderBox,
    PaddingBox,
}

impl Display for InsetClip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InsetClip::BorderBox => "border-box",
                InsetClip::PaddingBox => "padding-box",
            }
        )
    }
}

prop_responsive_enum!(InsetClipProp, InsetClip, Some("rt-r-clip"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InsetPadding {
    Current,
    Zero,
}

impl Display for InsetPadding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InsetPadding::Current => "inset",
                InsetPadding::Zero => "0",
            }
        )
    }
}

prop_optional_responsive_enum!(InsetPProp, InsetPadding, Some("rt-r-p"), None);
prop_optional_responsive_enum!(InsetPxProp, InsetPadding, Some("rt-r-px"), None);
prop_optional_responsive_enum!(InsetPyProp, InsetPadding, Some("rt-r-py"), None);
prop_optional_responsive_enum!(InsetPtProp, InsetPadding, Some("rt-r-pt"), None);
prop_optional_responsive_enum!(InsetPrProp, InsetPadding, Some("rt-r-pr"), None);
prop_optional_responsive_enum!(InsetPbProp, InsetPadding, Some("rt-r-pb"), None);
prop_optional_responsive_enum!(InsetPlProp, InsetPadding, Some("rt-r-pl"), None);
