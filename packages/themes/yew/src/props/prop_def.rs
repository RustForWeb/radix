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

pub enum StringValue {
    Defined(String),
    Arbitrary(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Responsive<T> {
    Value(T),
    Values(ResponsiveValues<T>),
}

pub type ResponsiveValues<T> = HashMap<Breakpoint, T>;

impl<T: Clone + Into<StringValue>> Responsive<T> {
    pub fn string_value(&self) -> Option<StringValue> {
        match self {
            Responsive::Value(value) => Some(value.clone().into()),
            Responsive::Values(_) => None,
        }
    }

    pub fn responsive_values(&self) -> Option<ResponsiveValues<StringValue>> {
        match self {
            Responsive::Value(_) => None,
            Responsive::Values(values) => Some(
                values
                    .iter()
                    .map(|(key, value)| (*key, value.clone().into()))
                    .collect(),
            ),
        }
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

    fn string_value(&self) -> Option<StringValue>;

    fn responsive_values(&self) -> Option<ResponsiveValues<StringValue>>;
}
