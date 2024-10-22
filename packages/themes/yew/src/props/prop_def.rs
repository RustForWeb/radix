use std::{
    collections::HashMap,
    fmt::{self, Display},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Breakpoint {
    Initial,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Breakpoint::Initial => "initial",
                Breakpoint::Xs => "xs",
                Breakpoint::Sm => "sm",
                Breakpoint::Md => "md",
                Breakpoint::Lg => "lg",
                Breakpoint::Xl => "xl",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropValue {
    Bool(bool),
    String(StringValue),
    Responsive(ResponsiveValues<StringValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringValue {
    Defined(String),
    Arbitrary(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Responsive<T> {
    Value(T),
    Values(ResponsiveValues<T>),
}

pub type ResponsiveValues<T> = HashMap<Breakpoint, T>;

impl<T: Clone + Into<StringValue>> Responsive<T> {
    pub fn value(&self) -> Option<PropValue> {
        Some(match self {
            Responsive::Value(value) => PropValue::String(value.clone().into()),
            Responsive::Values(values) => PropValue::Responsive(
                values
                    .iter()
                    .map(|(key, value)| (*key, value.clone().into()))
                    .collect(),
            ),
        })
    }
}

impl Responsive<String> {
    pub fn value_arbitrary(&self) -> Option<PropValue> {
        Some(match self {
            Responsive::Value(value) => PropValue::String(StringValue::Arbitrary(value.clone())),
            Responsive::Values(values) => PropValue::Responsive(
                values
                    .iter()
                    .map(|(key, value)| (*key, StringValue::Arbitrary(value.clone())))
                    .collect(),
            ),
        })
    }
}

pub enum PropDefType {
    Bool,
    String,
    Enum,
    EnumOrString,
}

pub trait PropDef {
    fn r#type(&self) -> PropDefType;

    fn class(&self) -> Option<&str>;

    fn responsive(&self) -> bool;

    fn custom_properties(&self) -> Option<&[&str]>;

    fn value(&self) -> Option<PropValue>;
}
