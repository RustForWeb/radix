use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Appearance {
    #[default]
    Inherit,
    Light,
    Dark,
}

impl Display for Appearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Appearance::Inherit => "inherit",
                Appearance::Light => "light",
                Appearance::Dark => "dark",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum PanelBackground {
    Solid,
    #[default]
    Translucent,
}

impl Display for PanelBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PanelBackground::Solid => "solid",
                PanelBackground::Translucent => "translucent",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Scaling {
    S90,
    S95,
    #[default]
    S100,
    S105,
    S110,
}

impl Display for Scaling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Scaling::S90 => "90%",
                Scaling::S95 => "95%",
                Scaling::S100 => "100%",
                Scaling::S105 => "105%",
                Scaling::S110 => "110%",
            }
        )
    }
}
