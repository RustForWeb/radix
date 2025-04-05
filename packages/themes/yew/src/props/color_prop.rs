use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, PropValue, StringValue, prop_optional_enum};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AccentColor {
    Gray,
    Gold,
    Bronze,
    Brown,
    Yellow,
    Amber,
    Orange,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    #[default]
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Lime,
    Mint,
    Sky,
}

impl Display for AccentColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccentColor::Gray => "gray",
                AccentColor::Gold => "gold",
                AccentColor::Bronze => "bronze",
                AccentColor::Brown => "brown",
                AccentColor::Yellow => "yellow",
                AccentColor::Amber => "amber",
                AccentColor::Orange => "orange",
                AccentColor::Tomato => "tomato",
                AccentColor::Red => "red",
                AccentColor::Ruby => "ruby",
                AccentColor::Crimson => "crimson",
                AccentColor::Pink => "pink",
                AccentColor::Plum => "plum",
                AccentColor::Purple => "purple",
                AccentColor::Violet => "violet",
                AccentColor::Iris => "iris",
                AccentColor::Indigo => "indigo",
                AccentColor::Blue => "blue",
                AccentColor::Cyan => "cyan",
                AccentColor::Teal => "teal",
                AccentColor::Jade => "jade",
                AccentColor::Green => "green",
                AccentColor::Grass => "grass",
                AccentColor::Lime => "lime",
                AccentColor::Mint => "mint",
                AccentColor::Sky => "sky",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GrayColor {
    #[default]
    Auto,
    Gray,
    Mauve,
    Slate,
    Sage,
    Olive,
    Sand,
}

impl Display for GrayColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GrayColor::Auto => "auto",
                GrayColor::Gray => "gray",
                GrayColor::Mauve => "mauve",
                GrayColor::Slate => "slate",
                GrayColor::Sage => "sage",
                GrayColor::Olive => "olive",
                GrayColor::Sand => "sand",
            }
        )
    }
}

prop_optional_enum!(ColorProp, AccentColor, None, None);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccentColorProp(pub Option<AccentColor>);

impl IntoPropValue<AccentColorProp> for AccentColor {
    fn into_prop_value(self) -> AccentColorProp {
        AccentColorProp(Some(self))
    }
}

impl PropDef for AccentColorProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        None
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(
            self.0.map(|value| value.to_string()).unwrap_or_default(),
        )))
    }
}
