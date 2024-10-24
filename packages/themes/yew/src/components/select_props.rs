use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, PropValue, StringValue};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SelectSize(u8);

impl Default for SelectSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for SelectSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for SelectSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Select size must be between 1 and 3, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelectSizeProp(pub SelectSize);

impl IntoPropValue<SelectSizeProp> for u8 {
    fn into_prop_value(self) -> SelectSizeProp {
        SelectSizeProp(self.try_into().unwrap())
    }
}

impl IntoPropValue<SelectSizeProp> for SelectSize {
    fn into_prop_value(self) -> SelectSizeProp {
        SelectSizeProp(self)
    }
}

impl PropDef for SelectSizeProp {
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SelectTriggerVariant {
    Classic,
    #[default]
    Surface,
    Soft,
    Ghost,
}

impl Display for SelectTriggerVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectTriggerVariant::Classic => "classic",
                SelectTriggerVariant::Surface => "surface",
                SelectTriggerVariant::Soft => "soft",
                SelectTriggerVariant::Ghost => "ghost",
            }
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelectTriggerVariantProp(pub SelectTriggerVariant);

impl IntoPropValue<SelectTriggerVariantProp> for SelectTriggerVariant {
    fn into_prop_value(self) -> SelectTriggerVariantProp {
        SelectTriggerVariantProp(self)
    }
}

impl PropDef for SelectTriggerVariantProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-variant")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(self.0.to_string())))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SelectContentVariant {
    #[default]
    Solid,
    Soft,
}

impl Display for SelectContentVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SelectContentVariant::Solid => "solid",
                SelectContentVariant::Soft => "soft",
            }
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelectContentVariantProp(pub SelectContentVariant);

impl IntoPropValue<SelectContentVariantProp> for SelectContentVariant {
    fn into_prop_value(self) -> SelectContentVariantProp {
        SelectContentVariantProp(self)
    }
}

impl PropDef for SelectContentVariantProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-variant")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(self.0.to_string())))
    }
}
