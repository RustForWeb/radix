use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ContainerSize(u8);

impl Default for ContainerSize {
    fn default() -> Self {
        Self(3)
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContainerSizeProp(pub ContainerSize);

impl IntoPropValue<ContainerSizeProp> for u8 {
    fn into_prop_value(self) -> ContainerSizeProp {
        ContainerSizeProp(self.try_into().unwrap())
    }
}

impl IntoPropValue<ContainerSizeProp> for ContainerSize {
    fn into_prop_value(self) -> ContainerSizeProp {
        ContainerSizeProp(self)
    }
}

impl PropDef for ContainerSizeProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-size")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(self.0.to_string())))
    }
}

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerDisplayProp(pub Option<Responsive<ContainerDisplay>>);

impl IntoPropValue<ContainerDisplayProp> for ContainerDisplay {
    fn into_prop_value(self) -> ContainerDisplayProp {
        ContainerDisplayProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<ContainerDisplayProp> for ResponsiveValues<ContainerDisplay> {
    fn into_prop_value(self) -> ContainerDisplayProp {
        ContainerDisplayProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for ContainerDisplayProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-display")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerAlignProp(pub Option<Responsive<ContainerAlign>>);

impl IntoPropValue<ContainerAlignProp> for ContainerAlign {
    fn into_prop_value(self) -> ContainerAlignProp {
        ContainerAlignProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<ContainerAlignProp> for ResponsiveValues<ContainerAlign> {
    fn into_prop_value(self) -> ContainerAlignProp {
        ContainerAlignProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for ContainerAlignProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-ai")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
