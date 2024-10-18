use crate::{
    helpers::{
        get_responsive::{get_responsive_classes, get_responsive_styles},
        merge_classes::merge_classes,
        // merge_styles::merge_styles,
    },
    props::prop_def::{PropDef, PropDefType},
};

pub fn extract_props(
    props: &[&dyn PropDef],
    props_class: Option<String>,
    _props_style: Option<String>,
) -> (String, String) {
    let mut class: String = "".to_string();
    let style: String = "".to_string();

    for prop in props {
        if prop.class().is_some() {
            match prop.r#type() {
                PropDefType::Enum => {
                    let prop_classes = get_responsive_classes(*prop);

                    class = merge_classes(class, prop_classes);
                }
                PropDefType::String | PropDefType::EnumOrString => {
                    let (prop_classes, _prop_custom_properties) = get_responsive_styles(*prop);

                    class = merge_classes(class, prop_classes);
                    // style = merge_styles(style, prop_custom_properties);
                }
                PropDefType::Bool => {
                    // TODO: handle responsive boolean props.
                    class = merge_classes(class, prop.class().map(|class| class.into()));
                }
            }
        }
    }

    (
        merge_classes(class, props_class),
        // merge_styles(style, props_style),
        style,
    )
}
