use std::ops::Deref;

pub trait AsClass {
    fn as_class(&self) -> Option<&str>;
}

impl AsClass for &str {
    fn as_class(&self) -> Option<&str> {
        Some(self)
    }
}

impl AsClass for String {
    fn as_class(&self) -> Option<&str> {
        Some(self)
    }
}

impl AsClass for Option<&str> {
    fn as_class(&self) -> Option<&str> {
        *self
    }
}

impl AsClass for Option<String> {
    fn as_class(&self) -> Option<&str> {
        self.as_ref().map(|value| value.as_str())
    }
}

impl<T: Deref<Target = Option<bool>>> AsClass for (&str, &T) {
    fn as_class(&self) -> Option<&str> {
        self.1.is_some_and(|value| value).then_some(self.0)
    }
}

pub fn merge_classes(values: &[&dyn AsClass]) -> String {
    let mut parts: Vec<&str> = vec![];

    for value in values {
        if let Some(class) = value.as_class() {
            parts.push(class);
        }
    }

    parts.join(" ")
}
