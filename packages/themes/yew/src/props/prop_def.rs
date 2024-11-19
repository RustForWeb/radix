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

impl<T: ToString> Responsive<T> {
    pub fn value_defined(&self) -> Option<PropValue> {
        Some(match self {
            Responsive::Value(value) => PropValue::String(StringValue::Defined(value.to_string())),
            Responsive::Values(values) => PropValue::Responsive(
                values
                    .iter()
                    .map(|(key, value)| (*key, StringValue::Defined(value.to_string())))
                    .collect(),
            ),
        })
    }

    pub fn value_arbitrary(&self) -> Option<PropValue> {
        Some(match self {
            Responsive::Value(value) => {
                PropValue::String(StringValue::Arbitrary(value.to_string()))
            }
            Responsive::Values(values) => PropValue::Responsive(
                values
                    .iter()
                    .map(|(key, value)| (*key, StringValue::Arbitrary(value.to_string())))
                    .collect(),
            ),
        })
    }
}

impl<T: Default> Default for Responsive<T> {
    fn default() -> Self {
        Self::Value(T::default())
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

    fn custom_properties(&self) -> Option<&[&str]>;

    fn value(&self) -> Option<PropValue>;
}

macro_rules! prop_bool {
    ($name:ident, $class:expr, $custom_properties:expr, $default:literal) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(pub bool);

        impl Default for $name {
            fn default() -> Self {
                Self($default)
            }
        }

        impl yew::html::IntoPropValue<$name> for bool {
            fn into_prop_value(self) -> $name {
                $name(self)
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Bool
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                Some($crate::PropValue::Bool(self.0))
            }
        }
    };
}

macro_rules! prop_enum {
    ($name:ident, $enum_name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub $enum_name);

        impl yew::html::IntoPropValue<$name> for $enum_name {
            fn into_prop_value(self) -> $name {
                $name(self)
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Enum
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                Some($crate::PropValue::String($crate::StringValue::Defined(
                    self.0.to_string(),
                )))
            }
        }
    };
}

macro_rules! prop_responsive_enum {
    ($name:ident, $enum_name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub $crate::Responsive<$enum_name>);

        impl yew::html::IntoPropValue<$name> for $enum_name {
            fn into_prop_value(self) -> $name {
                $name($crate::Responsive::Value(self))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<$enum_name> {
            fn into_prop_value(self) -> $name {
                $name($crate::Responsive::Values(self))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Enum
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.value_defined()
            }
        }
    };
}

macro_rules! prop_responsive_number_enum {
    ($name:ident, $enum_name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub $crate::Responsive<$enum_name>);

        impl yew::html::IntoPropValue<$name> for u8 {
            fn into_prop_value(self) -> $name {
                $name($crate::Responsive::Value(self.try_into().unwrap()))
            }
        }

        impl yew::html::IntoPropValue<$name> for $enum_name {
            fn into_prop_value(self) -> $name {
                $name($crate::Responsive::Value(self))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<u8> {
            fn into_prop_value(self) -> $name {
                $name($crate::Responsive::Values(
                    self.into_iter()
                        .map(|(key, value)| (key, value.try_into().unwrap()))
                        .collect(),
                ))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<$enum_name> {
            fn into_prop_value(self) -> $name {
                $name($crate::Responsive::Values(self))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Enum
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.value_defined()
            }
        }
    };
}

macro_rules! prop_optional_bool {
    ($name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub Option<bool>);

        impl std::ops::Deref for $name {
            type Target = Option<bool>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl yew::html::IntoPropValue<$name> for bool {
            fn into_prop_value(self) -> $name {
                $name(Some(self))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Bool
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.map($crate::PropValue::Bool)
            }
        }
    };
}

macro_rules! prop_optional_responsive_string {
    ($name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $name(pub Option<$crate::Responsive<String>>);

        impl yew::html::IntoPropValue<$name> for &str {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self.into())))
            }
        }

        impl yew::html::IntoPropValue<$name> for String {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self)))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<String> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(self)))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::String
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.as_ref().and_then(|value| value.value_arbitrary())
            }
        }
    };
}

macro_rules! prop_optional_enum {
    ($name:ident, $enum_name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub Option<$enum_name>);

        impl yew::html::IntoPropValue<$name> for $enum_name {
            fn into_prop_value(self) -> $name {
                $name(Some(self))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Enum
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.map(|value| {
                    $crate::PropValue::String($crate::StringValue::Defined(value.to_string()))
                })
            }
        }
    };
}

macro_rules! prop_optional_responsive_enum {
    ($name:ident, $enum_name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub Option<$crate::Responsive<$enum_name>>);

        impl yew::html::IntoPropValue<$name> for $enum_name {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self)))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<$enum_name> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(self)))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Enum
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.as_ref().and_then(|value| value.value_defined())
            }
        }
    };
}

macro_rules! prop_optional_responsive_number_enum {
    ($name:ident, $enum_name:ident, $class:expr, $custom_properties:expr) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub Option<$crate::Responsive<$enum_name>>);

        impl yew::html::IntoPropValue<$name> for u8 {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self.try_into().unwrap())))
            }
        }

        impl yew::html::IntoPropValue<$name> for $enum_name {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self)))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<u8> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(
                    self.into_iter()
                        .map(|(key, value)| (key, value.try_into().unwrap()))
                        .collect(),
                )))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<$enum_name> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(self)))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::Enum
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.as_ref().and_then(|value| value.value_defined())
            }
        }
    };
}

macro_rules! prop_optional_responsive_number_enum_or_string {
    ($name:ident, $enum_name:ident, $class:expr, $custom_properties:expr, $number_type:ty) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub Option<$crate::Responsive<$enum_name>>);

        impl yew::html::IntoPropValue<$name> for $number_type {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self.try_into().unwrap())))
            }
        }

        impl yew::html::IntoPropValue<$name> for &str {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self.into())))
            }
        }

        impl yew::html::IntoPropValue<$name> for String {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self.into())))
            }
        }

        impl yew::html::IntoPropValue<$name> for $enum_name {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Value(self)))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<$number_type> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(
                    self.into_iter()
                        .map(|(key, value)| (key, value.try_into().unwrap()))
                        .collect(),
                )))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<&str> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(
                    self.into_iter()
                        .map(|(key, value)| (key, value.into()))
                        .collect(),
                )))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<String> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(
                    self.into_iter()
                        .map(|(key, value)| (key, value.into()))
                        .collect(),
                )))
            }
        }

        impl yew::html::IntoPropValue<$name> for $crate::ResponsiveValues<$enum_name> {
            fn into_prop_value(self) -> $name {
                $name(Some($crate::Responsive::Values(self)))
            }
        }

        impl $crate::PropDef for $name {
            fn r#type(&self) -> $crate::PropDefType {
                $crate::PropDefType::EnumOrString
            }

            fn class(&self) -> Option<&str> {
                $class
            }

            fn custom_properties(&self) -> Option<&[&str]> {
                $custom_properties
            }

            fn value(&self) -> Option<$crate::PropValue> {
                self.0.as_ref().and_then(|value| value.value())
            }
        }
    };
}

pub(crate) use {
    prop_bool, prop_enum, prop_optional_bool, prop_optional_enum, prop_optional_responsive_enum,
    prop_optional_responsive_number_enum, prop_optional_responsive_number_enum_or_string,
    prop_optional_responsive_string, prop_responsive_enum, prop_responsive_number_enum,
};
