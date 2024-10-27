use std::fmt::{self, Display};

use crate::props::prop_def::{
    prop_optional_responsive_enum, prop_responsive_number_enum, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ContainerSize(u8);

impl Default for ContainerSize {
    fn default() -> Self {
        Self(4)
    }
}

impl Display for ContainerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for ContainerSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=4).contains(&value) {
            Err(format!(
                "Container size must be between 1 and 4, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(ContainerSizeProp, ContainerSize, Some("rt-r-size"), None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ContainerDisplay {
    None,
    Initial,
}

impl Display for ContainerDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ContainerDisplay::None => "none",
                ContainerDisplay::Initial => "flex",
            }
        )
    }
}

impl From<ContainerDisplay> for StringValue {
    fn from(value: ContainerDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(
    ContainerDisplayProp,
    ContainerDisplay,
    Some("rt-r-display"),
    None
);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ContainerAlign {
    Left,
    Center,
    Right,
}

impl Display for ContainerAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ContainerAlign::Left => "start",
                ContainerAlign::Center => "center",
                ContainerAlign::Right => "end",
            }
        )
    }
}

impl From<ContainerAlign> for StringValue {
    fn from(value: ContainerAlign) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(ContainerAlignProp, ContainerAlign, Some("rt-r-ai"), None);
