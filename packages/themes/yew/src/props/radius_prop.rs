use crate::props::prop_def::{PropDef, PropDefType};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Radius {
    None,
    Small,
    Medium,
    Large,
    Full,
}

impl PropDef for Option<Radius> {
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
