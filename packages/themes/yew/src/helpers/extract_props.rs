use std::collections::HashMap;

use crate::{
    helpers::{
        get_responsive::{get_responsive_classes, get_responsive_styles},
        merge_classes::merge_classes,
        merge_styles::merge_styles,
    },
    props::prop_def::{PropDef, PropDefType, PropValue},
};

pub fn extract_props(
    props: &[&dyn PropDef],
    props_class: Option<String>,
    props_style: Option<HashMap<String, String>>,
) -> (String, HashMap<String, String>) {
    let mut class: String = "".to_string();
    let mut style: HashMap<String, String> = HashMap::new();

    for prop in props {
        if prop.class().is_some() {
            match prop.r#type() {
                PropDefType::Enum => {
                    let prop_classes = get_responsive_classes(*prop, false);

                    class = merge_classes(&[&class, &prop_classes]);
                }
                PropDefType::String | PropDefType::EnumOrString => {
                    let (prop_classes, prop_custom_properties) = get_responsive_styles(*prop);

                    class = merge_classes(&[&class, &prop_classes]);
                    style = merge_styles(style, prop_custom_properties);
                }
                PropDefType::Bool => {
                    if let Some(PropValue::Bool(true)) = prop.value() {
                        // TODO: handle responsive boolean props.
                        class = merge_classes(&[&class, &prop.class()]);
                    }
                }
            }
        }
    }

    (
        merge_classes(&[&class, &props_class]),
        merge_styles(style, props_style),
    )
}
