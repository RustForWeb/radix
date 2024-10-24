use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GridAs {
    #[default]
    Div,
    Span,
}

impl Display for GridAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridAs::Div => "div",
                GridAs::Span => "span",
            }
        )
    }
}

impl From<GridAs> for StringValue {
    fn from(value: GridAs) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridAsProp(pub GridAs);

impl IntoPropValue<GridAsProp> for GridAs {
    fn into_prop_value(self) -> GridAsProp {
        GridAsProp(self)
    }
}

impl PropDef for GridAsProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        None
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        Some(PropValue::String(StringValue::Defined(self.0.to_string())))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridDisplay {
    None,
    InlineGrid,
    Grid,
}

impl Display for GridDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridDisplay::None => "none",
                GridDisplay::InlineGrid => "inline-grid",
                GridDisplay::Grid => "grid",
            }
        )
    }
}

impl From<GridDisplay> for StringValue {
    fn from(value: GridDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridDisplayProp(pub Option<Responsive<GridDisplay>>);

impl IntoPropValue<GridDisplayProp> for GridDisplay {
    fn into_prop_value(self) -> GridDisplayProp {
        GridDisplayProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridDisplayProp> for ResponsiveValues<GridDisplay> {
    fn into_prop_value(self) -> GridDisplayProp {
        GridDisplayProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridDisplayProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-display")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridAreasProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridAreasProp> for &str {
    fn into_prop_value(self) -> GridAreasProp {
        GridAreasProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridAreasProp> for String {
    fn into_prop_value(self) -> GridAreasProp {
        GridAreasProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridAreasProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridAreasProp {
        GridAreasProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridAreasProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gta")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-template-areas"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GridColumns {
    Defined(u8),
    Arbitrary(String),
}

impl Display for GridColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridColumns::Defined(value) => write!(f, "{}", value),
            GridColumns::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for GridColumns {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Grid columns must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for GridColumns {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for GridColumns {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<GridColumns> for StringValue {
    fn from(value: GridColumns) -> Self {
        match value {
            GridColumns::Defined(value) => StringValue::Defined(value.to_string()),
            GridColumns::Arbitrary(value) => {
                if let Ok(number) = value.parse::<u8>() {
                    StringValue::Arbitrary(format!("repeat({number}, minmax(0, 1fr))"))
                } else {
                    StringValue::Arbitrary(value.to_string())
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridColumnsProp(pub Option<Responsive<GridColumns>>);

impl IntoPropValue<GridColumnsProp> for GridColumns {
    fn into_prop_value(self) -> GridColumnsProp {
        GridColumnsProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridColumnsProp> for u8 {
    fn into_prop_value(self) -> GridColumnsProp {
        GridColumnsProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<GridColumnsProp> for &str {
    fn into_prop_value(self) -> GridColumnsProp {
        GridColumnsProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridColumnsProp> for String {
    fn into_prop_value(self) -> GridColumnsProp {
        GridColumnsProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridColumnsProp> for ResponsiveValues<GridColumns> {
    fn into_prop_value(self) -> GridColumnsProp {
        GridColumnsProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<GridColumnsProp> for ResponsiveValues<u8> {
    fn into_prop_value(self) -> GridColumnsProp {
        GridColumnsProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<GridColumnsProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridColumnsProp {
        GridColumnsProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for GridColumnsProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gtc")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-template-columns"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GridRows {
    Defined(u8),
    Arbitrary(String),
}

impl Display for GridRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridRows::Defined(value) => write!(f, "{}", value),
            GridRows::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for GridRows {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            Err(format!(
                "Grid rows must be between 1 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for GridRows {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for GridRows {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<GridRows> for StringValue {
    fn from(value: GridRows) -> Self {
        match value {
            GridRows::Defined(value) => StringValue::Defined(value.to_string()),
            GridRows::Arbitrary(value) => {
                if let Ok(number) = value.parse::<u8>() {
                    StringValue::Arbitrary(format!("repeat({number}, minmax(0, 1fr))"))
                } else {
                    StringValue::Arbitrary(value.to_string())
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridRowsProp(pub Option<Responsive<GridRows>>);

impl IntoPropValue<GridRowsProp> for GridRows {
    fn into_prop_value(self) -> GridRowsProp {
        GridRowsProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridRowsProp> for u8 {
    fn into_prop_value(self) -> GridRowsProp {
        GridRowsProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<GridRowsProp> for &str {
    fn into_prop_value(self) -> GridRowsProp {
        GridRowsProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridRowsProp> for String {
    fn into_prop_value(self) -> GridRowsProp {
        GridRowsProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridRowsProp> for ResponsiveValues<GridRows> {
    fn into_prop_value(self) -> GridRowsProp {
        GridRowsProp(Some(Responsive::Values(self)))
    }
}

impl IntoPropValue<GridRowsProp> for ResponsiveValues<u8> {
    fn into_prop_value(self) -> GridRowsProp {
        GridRowsProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.try_into().unwrap()))
                .collect(),
        )))
    }
}

impl IntoPropValue<GridRowsProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridRowsProp {
        GridRowsProp(Some(Responsive::Values(
            self.into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
        )))
    }
}

impl PropDef for GridRowsProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gtr")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-template-rows"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridFlow {
    Row,
    Column,
    Dense,
    RowDense,
    ColumnDense,
}

impl Display for GridFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridFlow::Row => "row",
                GridFlow::Column => "column",
                GridFlow::Dense => "dense",
                GridFlow::RowDense => "row-dense",
                GridFlow::ColumnDense => "column-dense",
            }
        )
    }
}

impl From<GridFlow> for StringValue {
    fn from(value: GridFlow) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridFlowProp(pub Option<Responsive<GridFlow>>);

impl IntoPropValue<GridFlowProp> for GridFlow {
    fn into_prop_value(self) -> GridFlowProp {
        GridFlowProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridFlowProp> for ResponsiveValues<GridFlow> {
    fn into_prop_value(self) -> GridFlowProp {
        GridFlowProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridFlowProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gaf")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridAlign {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl Display for GridAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridAlign::Start => "start",
                GridAlign::Center => "center",
                GridAlign::End => "end",
                GridAlign::Baseline => "baseline",
                GridAlign::Stretch => "strecht",
            }
        )
    }
}

impl From<GridAlign> for StringValue {
    fn from(value: GridAlign) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridAlignProp(pub Option<Responsive<GridAlign>>);

impl IntoPropValue<GridAlignProp> for GridAlign {
    fn into_prop_value(self) -> GridAlignProp {
        GridAlignProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridAlignProp> for ResponsiveValues<GridAlign> {
    fn into_prop_value(self) -> GridAlignProp {
        GridAlignProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridAlignProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-ai")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GridJustify {
    Start,
    Center,
    End,
    Between,
}

impl Display for GridJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridJustify::Start => "start",
                GridJustify::Center => "center",
                GridJustify::End => "end",
                GridJustify::Between => "space-between",
            }
        )
    }
}

impl From<GridJustify> for StringValue {
    fn from(value: GridJustify) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridJustifyProp(pub Option<Responsive<GridJustify>>);

impl IntoPropValue<GridJustifyProp> for GridJustify {
    fn into_prop_value(self) -> GridJustifyProp {
        GridJustifyProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridJustifyProp> for ResponsiveValues<GridJustify> {
    fn into_prop_value(self) -> GridJustifyProp {
        GridJustifyProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridJustifyProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-jc")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}
