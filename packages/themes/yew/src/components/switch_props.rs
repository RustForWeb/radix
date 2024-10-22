use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, PropValue, StringValue};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SwitchSize {
    S1,
    #[default]
    S2,
    S3,
}

impl Display for SwitchSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SwitchSize::S1 => "1",
                SwitchSize::S2 => "2",
                SwitchSize::S3 => "3",
            }
        )
    }
}

impl TryFrom<u8> for SwitchSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SwitchSize::S1),
            2 => Ok(SwitchSize::S2),
            3 => Ok(SwitchSize::S3),
            value => Err(format!(
                "Switch size must be between 0 and 3, but is {}.",
                value
            )),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SwitchSizeProp(pub SwitchSize);

impl IntoPropValue<SwitchSizeProp> for u8 {
    fn into_prop_value(self) -> SwitchSizeProp {
        SwitchSizeProp(self.try_into().unwrap())
    }
}

impl IntoPropValue<SwitchSizeProp> for SwitchSize {
    fn into_prop_value(self) -> SwitchSizeProp {
        SwitchSizeProp(self)
    }
}

impl PropDef for SwitchSizeProp {
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
pub enum SwitchVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

impl Display for SwitchVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SwitchVariant::Classic => "classic",
                SwitchVariant::Surface => "surface",
                SwitchVariant::Soft => "soft",
            }
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SwitchVariantProp(pub SwitchVariant);

impl IntoPropValue<SwitchVariantProp> for SwitchVariant {
    fn into_prop_value(self) -> SwitchVariantProp {
        SwitchVariantProp(self)
    }
}

impl PropDef for SwitchVariantProp {
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
