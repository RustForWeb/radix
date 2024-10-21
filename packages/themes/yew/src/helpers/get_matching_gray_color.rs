use crate::props::color_prop::{AccentColor, GrayColor};

pub fn get_matching_gray_color(accent_color: AccentColor) -> GrayColor {
    match accent_color {
        AccentColor::Tomato
        | AccentColor::Red
        | AccentColor::Ruby
        | AccentColor::Crimson
        | AccentColor::Pink
        | AccentColor::Plum
        | AccentColor::Purple
        | AccentColor::Violet => GrayColor::Mauve,
        AccentColor::Iris
        | AccentColor::Indigo
        | AccentColor::Blue
        | AccentColor::Sky
        | AccentColor::Cyan => GrayColor::Slate,
        AccentColor::Teal | AccentColor::Jade | AccentColor::Mint | AccentColor::Green => {
            GrayColor::Sage
        }
        AccentColor::Grass | AccentColor::Lime => GrayColor::Olive,
        AccentColor::Yellow
        | AccentColor::Amber
        | AccentColor::Orange
        | AccentColor::Brown
        | AccentColor::Gold
        | AccentColor::Bronze => GrayColor::Sand,
        AccentColor::Gray => GrayColor::Gray,
    }
}
