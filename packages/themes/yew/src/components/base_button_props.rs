use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BaseButtonSize(u8);

impl Default for BaseButtonSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for BaseButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for BaseButtonSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=4).contains(&value) {
            Err(format!(
                "Select size must be between 1 and 4, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BaseButtonSizeProp(pub Responsive<BaseButtonSize>);

impl IntoPropValue<BaseButtonSizeProp> for u8 {
    fn into_prop_value(self) -> BaseButtonSizeProp {
        BaseButtonSizeProp(Responsive::Value(self.try_into().unwrap()))
    }
}

impl IntoPropValue<BaseButtonSizeProp> for BaseButtonSize {
    fn into_prop_value(self) -> BaseButtonSizeProp {
        BaseButtonSizeProp(Responsive::Value(self))
    }
}

impl IntoPropValue<BaseButtonSizeProp> for ResponsiveValues<u8> {
    fn into_prop_value(self) -> BaseButtonSizeProp {
        BaseButtonSizeProp(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        ))
    }
}

impl IntoPropValue<BaseButtonSizeProp> for ResponsiveValues<BaseButtonSize> {
    fn into_prop_value(self) -> BaseButtonSizeProp {
        BaseButtonSizeProp(Responsive::Values(self))
    }
}

impl PropDef for BaseButtonSizeProp {
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
        self.0.value_defined()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BaseButtonVariant {
    Classic,
    #[default]
    Solid,
    Soft,
    Surface,
    Outline,
    Ghost,
}

impl Display for BaseButtonVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BaseButtonVariant::Classic => "classic",
                BaseButtonVariant::Solid => "solid",
                BaseButtonVariant::Soft => "soft",
                BaseButtonVariant::Surface => "surface",
                BaseButtonVariant::Outline => "outline",
                BaseButtonVariant::Ghost => "ghost",
            }
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BaseButtonVariantProp(pub BaseButtonVariant);

impl IntoPropValue<BaseButtonVariantProp> for BaseButtonVariant {
    fn into_prop_value(self) -> BaseButtonVariantProp {
        BaseButtonVariantProp(self)
    }
}

impl PropDef for BaseButtonVariantProp {
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BaseButtonLoadingProp(pub bool);

impl IntoPropValue<BaseButtonLoadingProp> for bool {
    fn into_prop_value(self) -> BaseButtonLoadingProp {
        BaseButtonLoadingProp(self)
    }
}

impl PropDef for BaseButtonLoadingProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Bool
    }

    fn class(&self) -> Option<&str> {
        Some("rt-loading")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::Bool(self.0))
    }
}
