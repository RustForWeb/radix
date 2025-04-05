use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::{
    components::base_button_props::{BaseButtonSize, BaseButtonSizeProp},
    props::prop_def::{Responsive, prop_bool, prop_responsive_number_enum},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpinnerSize(u8);

impl Default for SpinnerSize {
    fn default() -> Self {
        Self(2)
    }
}

impl Display for SpinnerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BaseButtonSize> for SpinnerSize {
    fn from(value: BaseButtonSize) -> Self {
        match value.0 {
            1 => Self(1),
            2 | 3 => Self(2),
            4 => Self(3),
            _ => unreachable!("Base button size is a number between 1 and 4."),
        }
    }
}

impl TryFrom<u8> for SpinnerSize {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=3).contains(&value) {
            Err(format!(
                "Spinner size must be between 1 and 3, but is {}.",
                value
            ))
        } else {
            Ok(Self(value))
        }
    }
}

prop_responsive_number_enum!(SpinnerSizeProp, SpinnerSize, Some("rt-r-size"), None);

impl IntoPropValue<SpinnerSizeProp> for BaseButtonSizeProp {
    fn into_prop_value(self) -> SpinnerSizeProp {
        match self.0 {
            Responsive::Value(size) => SpinnerSizeProp(Responsive::Value(size.into())),
            Responsive::Values(values) => SpinnerSizeProp(Responsive::Values(
                values
                    .into_iter()
                    .map(|(breakpoint, size)| (breakpoint, size.into()))
                    .collect(),
            )),
        }
    }
}

prop_bool!(SpinnerLoadingProp, None, None, true);
