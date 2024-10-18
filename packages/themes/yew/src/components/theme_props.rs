#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Appearance {
    Inherit,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PanelBackground {
    Solid,
    Translucent,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Scaling {
    S90,
    S95,
    S100,
    S105,
    S110,
}
