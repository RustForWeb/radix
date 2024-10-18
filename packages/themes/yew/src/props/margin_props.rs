use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, Responsive};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Margin {
    Value(i8),
    String(String),
}

impl TryFrom<i8> for Margin {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value <= -10 || value >= 10 {
            Err(format!(
                "Margin must be between -9 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Value(value))
        }
    }
}

impl From<&str> for Margin {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for Margin {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct M(pub Option<Responsive<Margin>>);

impl IntoPropValue<M> for Margin {
    fn into_prop_value(self) -> M {
        M(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<M> for i8 {
    fn into_prop_value(self) -> M {
        M(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<M> for &str {
    fn into_prop_value(self) -> M {
        M(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<M> for String {
    fn into_prop_value(self) -> M {
        M(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<M> for Responsive<Margin> {
    fn into_prop_value(self) -> M {
        M(Some(self))
    }
}

impl PropDef for M {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-m")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--m"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mx(pub Option<Responsive<Margin>>);

impl IntoPropValue<Mx> for Margin {
    fn into_prop_value(self) -> Mx {
        Mx(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Mx> for i8 {
    fn into_prop_value(self) -> Mx {
        Mx(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Mx> for &str {
    fn into_prop_value(self) -> Mx {
        Mx(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mx> for String {
    fn into_prop_value(self) -> Mx {
        Mx(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mx> for Responsive<Margin> {
    fn into_prop_value(self) -> Mx {
        Mx(Some(self))
    }
}

impl PropDef for Mx {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mx")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--ml", "--mr"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct My(pub Option<Responsive<Margin>>);

impl IntoPropValue<My> for Margin {
    fn into_prop_value(self) -> My {
        My(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<My> for i8 {
    fn into_prop_value(self) -> My {
        My(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<My> for &str {
    fn into_prop_value(self) -> My {
        My(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<My> for String {
    fn into_prop_value(self) -> My {
        My(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<My> for Responsive<Margin> {
    fn into_prop_value(self) -> My {
        My(Some(self))
    }
}

impl PropDef for My {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-my")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mt", "--mb"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mt(pub Option<Responsive<Margin>>);

impl IntoPropValue<Mt> for Margin {
    fn into_prop_value(self) -> Mt {
        Mt(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Mt> for i8 {
    fn into_prop_value(self) -> Mt {
        Mt(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Mt> for &str {
    fn into_prop_value(self) -> Mt {
        Mt(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mt> for String {
    fn into_prop_value(self) -> Mt {
        Mt(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mt> for Responsive<Margin> {
    fn into_prop_value(self) -> Mt {
        Mt(Some(self))
    }
}

impl PropDef for Mt {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mt")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mt"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mr(pub Option<Responsive<Margin>>);

impl IntoPropValue<Mr> for Margin {
    fn into_prop_value(self) -> Mr {
        Mr(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Mr> for i8 {
    fn into_prop_value(self) -> Mr {
        Mr(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Mr> for &str {
    fn into_prop_value(self) -> Mr {
        Mr(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mr> for String {
    fn into_prop_value(self) -> Mr {
        Mr(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mr> for Responsive<Margin> {
    fn into_prop_value(self) -> Mr {
        Mr(Some(self))
    }
}

impl PropDef for Mr {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mr")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mr"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mb(pub Option<Responsive<Margin>>);

impl IntoPropValue<Mb> for Margin {
    fn into_prop_value(self) -> Mb {
        Mb(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Mb> for i8 {
    fn into_prop_value(self) -> Mb {
        Mb(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Mb> for &str {
    fn into_prop_value(self) -> Mb {
        Mb(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mb> for String {
    fn into_prop_value(self) -> Mb {
        Mb(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Mb> for Responsive<Margin> {
    fn into_prop_value(self) -> Mb {
        Mb(Some(self))
    }
}

impl PropDef for Mb {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-mb")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--mb"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ml(pub Option<Responsive<Margin>>);

impl IntoPropValue<Ml> for Margin {
    fn into_prop_value(self) -> Ml {
        Ml(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Ml> for i8 {
    fn into_prop_value(self) -> Ml {
        Ml(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Ml> for &str {
    fn into_prop_value(self) -> Ml {
        Ml(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Ml> for String {
    fn into_prop_value(self) -> Ml {
        Ml(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Ml> for Responsive<Margin> {
    fn into_prop_value(self) -> Ml {
        Ml(Some(self))
    }
}

impl PropDef for Ml {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-ml")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--ml"])
    }
}
