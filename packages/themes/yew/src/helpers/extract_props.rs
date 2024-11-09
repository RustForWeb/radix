use yew::{prelude::classes, Classes};

use crate::{
    helpers::{
        get_responsive::{get_responsive_classes, get_responsive_styles},
        merge_styles::{merge_styles, Style},
    },
    props::prop_def::{PropDef, PropDefType, PropValue},
};

pub fn extract_props(
    props: &[&dyn PropDef],
    props_class: Option<String>,
    props_style: Option<Style>,
) -> (Classes, Style) {
    let mut class = Classes::new();
    let mut style = Style::new();

    for prop in props {
        if prop.class().is_some() {
            match prop.r#type() {
                PropDefType::Enum => {
                    let prop_classes = get_responsive_classes(*prop, false);

                    class = classes!(class, &prop_classes);
                }
                PropDefType::String | PropDefType::EnumOrString => {
                    let (prop_classes, prop_custom_properties) = get_responsive_styles(*prop);

                    class = classes!(class, &prop_classes);
                    style = merge_styles(style, prop_custom_properties);
                }
                PropDefType::Bool => {
                    if let Some(PropValue::Bool(true)) = prop.value() {
                        // TODO: handle responsive boolean props.
                        class = classes!(class, prop.class().map(|class| class.to_owned()));
                    }
                }
            }
        }
    }

    (
        classes!(class, &props_class),
        merge_styles(style, props_style),
    )
}
