use std::{
    collections::HashMap,
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

use yew::html::IntoPropValue;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Style(pub HashMap<String, String>);

impl Style {
    pub fn new() -> Self {
        Style(HashMap::new())
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(key, value)| format!("{key}: {value};"))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl Deref for Style {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Style {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<[(&str, &str); N]> for Style {
    fn from(value: [(&str, &str); N]) -> Self {
        Style(
            value
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<const N: usize> From<[(String, String); N]> for Style {
    fn from(value: [(String, String); N]) -> Self {
        Style(value.into())
    }
}

impl<const N: usize> IntoPropValue<Style> for [(&str, &str); N] {
    fn into_prop_value(self) -> Style {
        Style(
            self.into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<const N: usize> IntoPropValue<Style> for [(String, String); N] {
    fn into_prop_value(self) -> Style {
        Style(self.into())
    }
}

impl IntoPropValue<Style> for HashMap<&str, &str> {
    fn into_prop_value(self) -> Style {
        Style(
            self.into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl IntoPropValue<Style> for HashMap<String, String> {
    fn into_prop_value(self) -> Style {
        Style(self)
    }
}

pub fn merge_styles(a: Style, b: Option<Style>) -> Style {
    let mut result = HashMap::new();

    result.extend(a.0);

    if let Some(b) = b {
        result.extend(b.0);
    }

    Style(result)
}
