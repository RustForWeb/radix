use std::fmt::{self, Display};

use yew::html::IntoPropValue;

use crate::{
    props::prop_def::{PropDef, PropDefType, PropValue, Responsive},
    ResponsiveValues, StringValue,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FlexAs {
    #[default]
    Div,
    Span,
}

impl Display for FlexAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexAs::Div => "div",
                FlexAs::Span => "span",
            }
        )
    }
}

impl From<FlexAs> for StringValue {
    fn from(value: FlexAs) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexAsProp(pub FlexAs);

impl IntoPropValue<FlexAsProp> for FlexAs {
    fn into_prop_value(self) -> FlexAsProp {
        FlexAsProp(self)
    }
}

impl PropDef for FlexAsProp {
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
pub enum FlexDisplay {
    None,
    InlineFlex,
    Flex,
}

impl Display for FlexDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexDisplay::None => "none",
                FlexDisplay::InlineFlex => "inline-flex",
                FlexDisplay::Flex => "flex",
            }
        )
    }
}

impl From<FlexDisplay> for StringValue {
    fn from(value: FlexDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexDisplayProp(pub Option<Responsive<FlexDisplay>>);

impl IntoPropValue<FlexDisplayProp> for FlexDisplay {
    fn into_prop_value(self) -> FlexDisplayProp {
        FlexDisplayProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexDisplayProp> for ResponsiveValues<FlexDisplay> {
    fn into_prop_value(self) -> FlexDisplayProp {
        FlexDisplayProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexDisplayProp {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexDirection::Row => "row",
                FlexDirection::Column => "column",
                FlexDirection::RowReverse => "row-reverse",
                FlexDirection::ColumnReverse => "column-reverse",
            }
        )
    }
}

impl From<FlexDirection> for StringValue {
    fn from(value: FlexDirection) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexDirectionProp(pub Option<Responsive<FlexDirection>>);

impl IntoPropValue<FlexDirectionProp> for FlexDirection {
    fn into_prop_value(self) -> FlexDirectionProp {
        FlexDirectionProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexDirectionProp> for ResponsiveValues<FlexDirection> {
    fn into_prop_value(self) -> FlexDirectionProp {
        FlexDirectionProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexDirectionProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-fd")
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
pub enum FlexAlign {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl Display for FlexAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexAlign::Start => "start",
                FlexAlign::Center => "center",
                FlexAlign::End => "end",
                FlexAlign::Baseline => "baseline",
                FlexAlign::Stretch => "strecht",
            }
        )
    }
}

impl From<FlexAlign> for StringValue {
    fn from(value: FlexAlign) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexAlignProp(pub Option<Responsive<FlexAlign>>);

impl IntoPropValue<FlexAlignProp> for FlexAlign {
    fn into_prop_value(self) -> FlexAlignProp {
        FlexAlignProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexAlignProp> for ResponsiveValues<FlexAlign> {
    fn into_prop_value(self) -> FlexAlignProp {
        FlexAlignProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexAlignProp {
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
pub enum FlexJustify {
    Start,
    Center,
    End,
    Between,
}

impl Display for FlexJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexJustify::Start => "start",
                FlexJustify::Center => "center",
                FlexJustify::End => "end",
                FlexJustify::Between => "space-between",
            }
        )
    }
}

impl From<FlexJustify> for StringValue {
    fn from(value: FlexJustify) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexJustifyProp(pub Option<Responsive<FlexJustify>>);

impl IntoPropValue<FlexJustifyProp> for FlexJustify {
    fn into_prop_value(self) -> FlexJustifyProp {
        FlexJustifyProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexJustifyProp> for ResponsiveValues<FlexJustify> {
    fn into_prop_value(self) -> FlexJustifyProp {
        FlexJustifyProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexJustifyProp {
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

impl Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexWrap::Nowrap => "nowrap",
                FlexWrap::Wrap => "wrap",
                FlexWrap::WrapReverse => "wrap-reverse",
            }
        )
    }
}

impl From<FlexWrap> for StringValue {
    fn from(value: FlexWrap) -> Self {
        StringValue::Defined(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexWrapProp(pub Option<Responsive<FlexWrap>>);

impl IntoPropValue<FlexWrapProp> for FlexWrap {
    fn into_prop_value(self) -> FlexWrapProp {
        FlexWrapProp(Some(Responsive::Value(self)))
    }
}

impl IntoPropValue<FlexWrapProp> for ResponsiveValues<FlexWrap> {
    fn into_prop_value(self) -> FlexWrapProp {
        FlexWrapProp(Some(Responsive::Values(self)))
    }
}

impl PropDef for FlexWrapProp {
    fn r#type(&self) -> PropDefType {
        PropDefType::Enum
    }

    fn class(&self) -> Option<&str> {
        Some("rt-r-fw")
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
