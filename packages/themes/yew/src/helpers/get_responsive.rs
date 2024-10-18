use crate::props::prop_def::PropDef;

pub fn get_responsive_styles(_prop: &dyn PropDef) -> (Option<String>, &[&str]) {
    (None, &[])
}

pub fn get_responsive_classes(_prop: &dyn PropDef) -> Option<String> {
    None
}
