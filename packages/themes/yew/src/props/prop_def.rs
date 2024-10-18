pub enum Breakpoint {
    Initial,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Responsive<T> {
    Value(T),
    Values(ResponsiveValues<T>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponsiveValues<T> {
    pub initial: Option<T>,
    pub xs: Option<T>,
    pub sm: Option<T>,
    pub md: Option<T>,
    pub lg: Option<T>,
    pub xl: Option<T>,
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
}
