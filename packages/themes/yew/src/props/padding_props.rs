use yew::html::IntoPropValue;

use crate::props::prop_def::{PropDef, PropDefType, Responsive};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Padding {
    Value(i8),
    String(String),
}

impl TryFrom<i8> for Padding {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value <= -10 || value >= 10 {
            Err(format!(
                "Padding must be between -9 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Value(value))
        }
    }
}

impl From<&str> for Padding {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for Padding {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct P(pub Option<Responsive<Padding>>);

impl IntoPropValue<P> for Padding {
    fn into_prop_value(self) -> P {
        P(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<P> for i8 {
    fn into_prop_value(self) -> P {
        P(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<P> for &str {
    fn into_prop_value(self) -> P {
        P(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<P> for String {
    fn into_prop_value(self) -> P {
        P(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<P> for Responsive<Padding> {
    fn into_prop_value(self) -> P {
        P(Some(self))
    }
}

impl PropDef for P {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-p")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--p"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Px(pub Option<Responsive<Padding>>);

impl IntoPropValue<Px> for Padding {
    fn into_prop_value(self) -> Px {
        Px(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Px> for i8 {
    fn into_prop_value(self) -> Px {
        Px(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Px> for &str {
    fn into_prop_value(self) -> Px {
        Px(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Px> for String {
    fn into_prop_value(self) -> Px {
        Px(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Px> for Responsive<Padding> {
    fn into_prop_value(self) -> Px {
        Px(Some(self))
    }
}

impl PropDef for Px {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-px")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pl", "--pr"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Py(pub Option<Responsive<Padding>>);

impl IntoPropValue<Py> for Padding {
    fn into_prop_value(self) -> Py {
        Py(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Py> for i8 {
    fn into_prop_value(self) -> Py {
        Py(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Py> for &str {
    fn into_prop_value(self) -> Py {
        Py(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Py> for String {
    fn into_prop_value(self) -> Py {
        Py(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Py> for Responsive<Padding> {
    fn into_prop_value(self) -> Py {
        Py(Some(self))
    }
}

impl PropDef for Py {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-py")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pt", "--pb"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pt(pub Option<Responsive<Padding>>);

impl IntoPropValue<Pt> for Padding {
    fn into_prop_value(self) -> Pt {
        Pt(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Pt> for i8 {
    fn into_prop_value(self) -> Pt {
        Pt(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Pt> for &str {
    fn into_prop_value(self) -> Pt {
        Pt(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pt> for String {
    fn into_prop_value(self) -> Pt {
        Pt(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pt> for Responsive<Padding> {
    fn into_prop_value(self) -> Pt {
        Pt(Some(self))
    }
}

impl PropDef for Pt {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pt")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pt"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pr(pub Option<Responsive<Padding>>);

impl IntoPropValue<Pr> for Padding {
    fn into_prop_value(self) -> Pr {
        Pr(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Pr> for i8 {
    fn into_prop_value(self) -> Pr {
        Pr(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Pr> for &str {
    fn into_prop_value(self) -> Pr {
        Pr(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pr> for String {
    fn into_prop_value(self) -> Pr {
        Pr(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pr> for Responsive<Padding> {
    fn into_prop_value(self) -> Pr {
        Pr(Some(self))
    }
}

impl PropDef for Pr {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pr")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pr"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pb(pub Option<Responsive<Padding>>);

impl IntoPropValue<Pb> for Padding {
    fn into_prop_value(self) -> Pb {
        Pb(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Pb> for i8 {
    fn into_prop_value(self) -> Pb {
        Pb(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Pb> for &str {
    fn into_prop_value(self) -> Pb {
        Pb(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pb> for String {
    fn into_prop_value(self) -> Pb {
        Pb(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pb> for Responsive<Padding> {
    fn into_prop_value(self) -> Pb {
        Pb(Some(self))
    }
}

impl PropDef for Pb {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pb")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pb"])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pl(pub Option<Responsive<Padding>>);

impl IntoPropValue<Pl> for Padding {
    fn into_prop_value(self) -> Pl {
        Pl(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<Pl> for i8 {
    fn into_prop_value(self) -> Pl {
        Pl(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<Pl> for &str {
    fn into_prop_value(self) -> Pl {
        Pl(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pl> for String {
    fn into_prop_value(self) -> Pl {
        Pl(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<Pl> for Responsive<Padding> {
    fn into_prop_value(self) -> Pl {
        Pl(Some(self))
    }
}

impl PropDef for Pl {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-pl")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--pl"])
    }
}
