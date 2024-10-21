use std::collections::HashMap;

use crate::props::prop_def::{Breakpoint, PropDef, PropValue, StringValue};

pub fn get_responsive_styles(
    prop: &dyn PropDef,
) -> (Option<String>, Option<HashMap<String, String>>) {
    let responsive_class = get_responsive_classes(prop, true);
    let responsive_custom_properties = get_responsive_custom_properties(prop);

    (responsive_class, responsive_custom_properties)
}

pub fn get_responsive_classes(prop: &dyn PropDef, allow_arbitrary_values: bool) -> Option<String> {
    let class = prop.class().expect("Class should exist.");

    let value = prop.value();

    if let Some(value) = value {
        if let PropValue::Bool(false) = value {
            return None;
        }

        if let PropValue::String(StringValue::Defined(value)) = value {
            return Some(get_base_class(prop, value));
        }

        if let PropValue::Responsive(values) = value {
            let mut classes = vec![];

            for (bp, value) in values {
                match value {
                    StringValue::Defined(value) => {
                        let base_class = get_base_class(prop, value);
                        let bp_class = if bp == Breakpoint::Initial {
                            base_class
                        } else {
                            format!("{bp}:{base_class}")
                        };
                        classes.push(bp_class);
                    }
                    StringValue::Arbitrary(_) => {
                        if allow_arbitrary_values {
                            // TODO: allow arbitrary check
                            let bp_class = if bp == Breakpoint::Initial {
                                class.to_string()
                            } else {
                                format!("{bp}:{class}")
                            };
                            classes.push(bp_class);
                        }
                    }
                }
            }

            return Some(classes.join(" "));
        }

        if allow_arbitrary_values {
            Some(class.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

fn get_base_class(prop: &dyn PropDef, matched_value: String) -> String {
    let class = prop.class().expect("Class should exist.");
    let is_negative = matched_value.starts_with('-');
    let minus = if is_negative { "-" } else { "" };
    let absolute_value = if is_negative {
        matched_value[1..].to_string()
    } else {
        matched_value
    };
    format!("{minus}{class}-{absolute_value}")
}

pub fn get_responsive_custom_properties(prop: &dyn PropDef) -> Option<HashMap<String, String>> {
    let custom_properties = prop
        .custom_properties()
        .expect("Custom properties should exist.");

    let value = prop.value();

    if let Some(value) = value {
        if let PropValue::Bool(false) = value {
            return None;
        }

        // Don't generate custom properties if the value is not arbitrary.
        if let PropValue::String(StringValue::Defined(_)) = value {
            return None;
        }

        let mut styles: HashMap<String, String> = HashMap::new();

        if let PropValue::String(StringValue::Arbitrary(value)) = &value {
            styles = custom_properties
                .iter()
                .map(|custom_property| (custom_property.to_string(), value.clone()))
                .collect();
        }

        if let PropValue::Responsive(values) = value {
            for (bp, value) in values {
                match value {
                    // Don't generate a custom property if the value is not arbitrary.
                    StringValue::Defined(_) => {}
                    StringValue::Arbitrary(value) => {
                        styles.extend(custom_properties.iter().map(|custom_property| {
                            let bp_property = if bp == Breakpoint::Initial {
                                custom_property.to_string()
                            } else {
                                format!("{custom_property}-{bp}")
                            };

                            (bp_property, value.clone())
                        }));
                    }
                }
            }
        }

        Some(styles)
    } else {
        None
    }
}
