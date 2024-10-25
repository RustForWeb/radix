use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum HeadingAs {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl Display for HeadingAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HeadingAs::H1 => "h1",
                HeadingAs::H2 => "h2",
                HeadingAs::H3 => "h3",
                HeadingAs::H4 => "h4",
                HeadingAs::H5 => "h5",
                HeadingAs::H6 => "h6",
            }
        )
    }
}

impl From<HeadingAs> for StringValue {
    fn from(value: HeadingAs) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeadingAsProp(pub HeadingAs);

impl IntoPropValue<HeadingAsProp> for HeadingAs {
    fn into_prop_value(self) -> HeadingAsProp {
        HeadingAsProp(self)
    }
}

impl PropDef for HeadingAsProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        None
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
pub struct HeadingSize(u8);

impl Default for HeadingSize {
    fn default() -> Self {
        Self(6)
    }
}

impl Display for HeadingSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for HeadingSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Heading size must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HeadingSizeProp(pub Responsive<HeadingSize>);

impl IntoPropValue<HeadingSizeProp> for u8 {
    fn into_prop_value(self) -> HeadingSizeProp {
        HeadingSizeProp(Responsive::Value(self.try_into().unwrap()))
    }
}

impl IntoPropValue<HeadingSizeProp> for HeadingSize {
    fn into_prop_value(self) -> HeadingSizeProp {
        HeadingSizeProp(Responsive::Value(self))
    }
}

impl IntoPropValue<HeadingSizeProp> for ResponsiveValues<u8> {
    fn into_prop_value(self) -> HeadingSizeProp {
        HeadingSizeProp(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        ))
    }
}

impl IntoPropValue<HeadingSizeProp> for ResponsiveValues<HeadingSize> {
    fn into_prop_value(self) -> HeadingSizeProp {
        HeadingSizeProp(Responsive::Values(self))
    }
}

impl PropDef for HeadingSizeProp {
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
