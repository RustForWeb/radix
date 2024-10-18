use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl PropDef for AccentColor {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        None
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GrayColor {
    Auto,
    Gray,
    Mauve,
    Slate,
    Sage,
    Olive,
    Sand,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Color(pub Option<AccentColor>);

impl PropDef for Color {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        None
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }
}

impl IntoPropValue<Color> for AccentColor {
    fn into_prop_value(self) -> Color {
        Color(Some(self))
    }
}
