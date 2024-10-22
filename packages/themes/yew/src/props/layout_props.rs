use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::props::prop_def::{
    PropDef, PropDefType, PropValue, Responsive, ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::Static => "static",
                Position::Relative => "relative",
                Position::Absolute => "absolute",
                Position::Fixed => "fixed",
                Position::Sticky => "sticky",
            }
        )
    }
}

impl From<Position> for StringValue {
    fn from(value: Position) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PositionProp(pub Option<Responsive<Position>>);

impl IntoPropValue<PositionProp> for Position {
    fn into_prop_value(self) -> PositionProp {
        PositionProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<PositionProp> for ResponsiveValues<Position> {
    fn into_prop_value(self) -> PositionProp {
        PositionProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for PositionProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-position")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PositionEdge {
    Defined(i8),
    Arbitrary(String),
}

impl Display for PositionEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionEdge::Defined(value) => write!(f, "{}", value),
            PositionEdge::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<i8> for PositionEdge {
    type Error = String;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if !(-9..=9).contains(&value) {
            Err(format!(
                "Position edge must be between -9 and 9, but is {}.",
                value
            ))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for PositionEdge {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for PositionEdge {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<PositionEdge> for StringValue {
    fn from(value: PositionEdge) -> Self {
        match value {
            PositionEdge::Defined(value) => StringValue::Defined(value.to_string()),
            PositionEdge::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InsetProp(pub Option<Responsive<PositionEdge>>);

impl IntoPropValue<InsetProp> for PositionEdge {
    fn into_prop_value(self) -> InsetProp {
        InsetProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<InsetProp> for i8 {
    fn into_prop_value(self) -> InsetProp {
        InsetProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<InsetProp> for &str {
    fn into_prop_value(self) -> InsetProp {
        InsetProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<InsetProp> for String {
    fn into_prop_value(self) -> InsetProp {
        InsetProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<InsetProp> for ResponsiveValues<PositionEdge> {
    fn into_prop_value(self) -> InsetProp {
        InsetProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for InsetProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-inset")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--inset"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TopProp(pub Option<Responsive<PositionEdge>>);

impl IntoPropValue<TopProp> for PositionEdge {
    fn into_prop_value(self) -> TopProp {
        TopProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<TopProp> for i8 {
    fn into_prop_value(self) -> TopProp {
        TopProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<TopProp> for &str {
    fn into_prop_value(self) -> TopProp {
        TopProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<TopProp> for String {
    fn into_prop_value(self) -> TopProp {
        TopProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<TopProp> for ResponsiveValues<PositionEdge> {
    fn into_prop_value(self) -> TopProp {
        TopProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for TopProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-top")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--top"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RightProp(pub Option<Responsive<PositionEdge>>);

impl IntoPropValue<RightProp> for PositionEdge {
    fn into_prop_value(self) -> RightProp {
        RightProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<RightProp> for i8 {
    fn into_prop_value(self) -> RightProp {
        RightProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<RightProp> for &str {
    fn into_prop_value(self) -> RightProp {
        RightProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<RightProp> for String {
    fn into_prop_value(self) -> RightProp {
        RightProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<RightProp> for ResponsiveValues<PositionEdge> {
    fn into_prop_value(self) -> RightProp {
        RightProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for RightProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-right")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--right"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BottomProp(pub Option<Responsive<PositionEdge>>);

impl IntoPropValue<BottomProp> for PositionEdge {
    fn into_prop_value(self) -> BottomProp {
        BottomProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<BottomProp> for i8 {
    fn into_prop_value(self) -> BottomProp {
        BottomProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<BottomProp> for &str {
    fn into_prop_value(self) -> BottomProp {
        BottomProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<BottomProp> for String {
    fn into_prop_value(self) -> BottomProp {
        BottomProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<BottomProp> for ResponsiveValues<PositionEdge> {
    fn into_prop_value(self) -> BottomProp {
        BottomProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for BottomProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-bottom")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--bottom"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LeftProp(pub Option<Responsive<PositionEdge>>);

impl IntoPropValue<LeftProp> for PositionEdge {
    fn into_prop_value(self) -> LeftProp {
        LeftProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<LeftProp> for i8 {
    fn into_prop_value(self) -> LeftProp {
        LeftProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<LeftProp> for &str {
    fn into_prop_value(self) -> LeftProp {
        LeftProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<LeftProp> for String {
    fn into_prop_value(self) -> LeftProp {
        LeftProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<LeftProp> for ResponsiveValues<PositionEdge> {
    fn into_prop_value(self) -> LeftProp {
        LeftProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for LeftProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-left")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--left"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

impl Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Overflow::Visible => "visible",
                Overflow::Hidden => "hidden",
                Overflow::Clip => "clip",
                Overflow::Scroll => "scroll",
                Overflow::Auto => "auto",
            }
        )
    }
}

impl From<Overflow> for StringValue {
    fn from(value: Overflow) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OverflowProp(pub Option<Responsive<Overflow>>);

impl IntoPropValue<OverflowProp> for Overflow {
    fn into_prop_value(self) -> OverflowProp {
        OverflowProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<OverflowProp> for ResponsiveValues<Overflow> {
    fn into_prop_value(self) -> OverflowProp {
        OverflowProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for OverflowProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-overflow")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OverflowXProp(pub Option<Responsive<Overflow>>);

impl IntoPropValue<OverflowXProp> for Overflow {
    fn into_prop_value(self) -> OverflowXProp {
        OverflowXProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<OverflowXProp> for ResponsiveValues<Overflow> {
    fn into_prop_value(self) -> OverflowXProp {
        OverflowXProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for OverflowXProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-ox")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OverflowYProp(pub Option<Responsive<Overflow>>);

impl IntoPropValue<OverflowYProp> for Overflow {
    fn into_prop_value(self) -> OverflowYProp {
        OverflowYProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<OverflowYProp> for ResponsiveValues<Overflow> {
    fn into_prop_value(self) -> OverflowYProp {
        OverflowYProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for OverflowYProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-oy")
    }

    fn responsive(&self) -> bool {
        false
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        None
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexBasisProp(pub Option<Responsive<String>>);

impl IntoPropValue<FlexBasisProp> for &str {
    fn into_prop_value(self) -> FlexBasisProp {
        FlexBasisProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<FlexBasisProp> for String {
    fn into_prop_value(self) -> FlexBasisProp {
        FlexBasisProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexBasisProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> FlexBasisProp {
        FlexBasisProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexBasisProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-fb")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--flex-basis"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FlexShrink {
    Defined(u8),
    Arbitrary(String),
}

impl Display for FlexShrink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexShrink::Defined(value) => write!(f, "{}", value),
            FlexShrink::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for FlexShrink {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=1).contains(&value) {
            Err(format!("Flex shrink must be 0 or 1, but is {}.", value))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for FlexShrink {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for FlexShrink {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<FlexShrink> for StringValue {
    fn from(value: FlexShrink) -> Self {
        match value {
            FlexShrink::Defined(value) => StringValue::Defined(value.to_string()),
            FlexShrink::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexShrinkProp(pub Option<Responsive<FlexShrink>>);

impl IntoPropValue<FlexShrinkProp> for FlexShrink {
    fn into_prop_value(self) -> FlexShrinkProp {
        FlexShrinkProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexShrinkProp> for u8 {
    fn into_prop_value(self) -> FlexShrinkProp {
        FlexShrinkProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<FlexShrinkProp> for &str {
    fn into_prop_value(self) -> FlexShrinkProp {
        FlexShrinkProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<FlexShrinkProp> for String {
    fn into_prop_value(self) -> FlexShrinkProp {
        FlexShrinkProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<FlexShrinkProp> for ResponsiveValues<FlexShrink> {
    fn into_prop_value(self) -> FlexShrinkProp {
        FlexShrinkProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexShrinkProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-fs")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--flex-shrink"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FlexGrow {
    Defined(u8),
    Arbitrary(String),
}

impl Display for FlexGrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexGrow::Defined(value) => write!(f, "{}", value),
            FlexGrow::Arbitrary(value) => write!(f, "{}", value),
        }
    }
}

impl TryFrom<u8> for FlexGrow {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=1).contains(&value) {
            Err(format!("Flex grow must be 0 or 1, but is {}.", value))
        } else {
            Ok(Self::Defined(value))
        }
    }
}

impl From<&str> for FlexGrow {
    fn from(value: &str) -> Self {
        Self::Arbitrary(value.into())
    }
}

impl From<String> for FlexGrow {
    fn from(value: String) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<FlexGrow> for StringValue {
    fn from(value: FlexGrow) -> Self {
        match value {
            FlexGrow::Defined(value) => StringValue::Defined(value.to_string()),
            FlexGrow::Arbitrary(value) => StringValue::Arbitrary(value.to_string()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexGrowProp(pub Option<Responsive<FlexGrow>>);

impl IntoPropValue<FlexGrowProp> for FlexGrow {
    fn into_prop_value(self) -> FlexGrowProp {
        FlexGrowProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexGrowProp> for u8 {
    fn into_prop_value(self) -> FlexGrowProp {
        FlexGrowProp(Some(Responsive::Value(self.try_into().unwrap())))
    }
}

impl IntoPropValue<FlexGrowProp> for &str {
    fn into_prop_value(self) -> FlexGrowProp {
        FlexGrowProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<FlexGrowProp> for String {
    fn into_prop_value(self) -> FlexGrowProp {
        FlexGrowProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<FlexGrowProp> for ResponsiveValues<FlexGrow> {
    fn into_prop_value(self) -> FlexGrowProp {
        FlexGrowProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexGrowProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::EnumOrString
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-fg")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--flex-grow"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridAreaProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridAreaProp> for &str {
    fn into_prop_value(self) -> GridAreaProp {
        GridAreaProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridAreaProp> for String {
    fn into_prop_value(self) -> GridAreaProp {
        GridAreaProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridAreaProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridAreaProp {
        GridAreaProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridAreaProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-ga")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-area"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridColumnProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridColumnProp> for &str {
    fn into_prop_value(self) -> GridColumnProp {
        GridColumnProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridColumnProp> for String {
    fn into_prop_value(self) -> GridColumnProp {
        GridColumnProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridColumnProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridColumnProp {
        GridColumnProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridColumnProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gc")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-column"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridColumnStartProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridColumnStartProp> for &str {
    fn into_prop_value(self) -> GridColumnStartProp {
        GridColumnStartProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridColumnStartProp> for String {
    fn into_prop_value(self) -> GridColumnStartProp {
        GridColumnStartProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridColumnStartProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridColumnStartProp {
        GridColumnStartProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridColumnStartProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gcs")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-column-start"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridColumnEndProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridColumnEndProp> for &str {
    fn into_prop_value(self) -> GridColumnEndProp {
        GridColumnEndProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridColumnEndProp> for String {
    fn into_prop_value(self) -> GridColumnEndProp {
        GridColumnEndProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridColumnEndProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridColumnEndProp {
        GridColumnEndProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridColumnEndProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gce")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-column-end"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridRowProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridRowProp> for &str {
    fn into_prop_value(self) -> GridRowProp {
        GridRowProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridRowProp> for String {
    fn into_prop_value(self) -> GridRowProp {
        GridRowProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridRowProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridRowProp {
        GridRowProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridRowProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gr")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-row"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridRowStartProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridRowStartProp> for &str {
    fn into_prop_value(self) -> GridRowStartProp {
        GridRowStartProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridRowStartProp> for String {
    fn into_prop_value(self) -> GridRowStartProp {
        GridRowStartProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridRowStartProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridRowStartProp {
        GridRowStartProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridRowStartProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-grs")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-row-start"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridRowEndProp(pub Option<Responsive<String>>);

impl IntoPropValue<GridRowEndProp> for &str {
    fn into_prop_value(self) -> GridRowEndProp {
        GridRowEndProp(Some(Responsive::Value(self.into())))
    }
}

impl IntoPropValue<GridRowEndProp> for String {
    fn into_prop_value(self) -> GridRowEndProp {
        GridRowEndProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<GridRowEndProp> for ResponsiveValues<String> {
    fn into_prop_value(self) -> GridRowEndProp {
        GridRowEndProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for GridRowEndProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::String
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-gre")
    }

    fn responsive(&self) -> bool {
        true
    }

    fn custom_properties(&self) -> Option<&[&str]> {
        Some(&["--grid-row-end"])
    }

    fn value(&self) -> Option<PropValue> {
        self.0.as_ref().and_then(|value| value.value_arbitrary())
    }
}
