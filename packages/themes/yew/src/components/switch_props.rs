use crate::props::prop_def::{PropDef, PropDefType};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SwitchSize {
    S1,
    #[default]
    S2,
    S3,
}

impl PropDef for SwitchSize {
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
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum SwitchVariant {
    Classic,
    #[default]
    Surface,
    Soft,
}

impl PropDef for SwitchVariant {
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
}
