use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent, struct_component};
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct VisuallyHiddenProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<VisuallyHiddenChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "span")]
pub struct VisuallyHiddenChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn VisuallyHidden(props: &VisuallyHiddenProps) -> Html {
    let child_props = VisuallyHiddenChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone().with_defaults([
            // See https://github.com/twbs/bootstrap/blob/master/scss/mixins/_screen-reader.scss.
            ("position", "absolute"),
            ("border", "0px"),
            ("width", "1px"),
            ("height", "1px"),
            ("padding", "0px"),
            ("margin", "-1px"),
            ("overflow", "hidden"),
            ("clip", "rect(0, 0, 0, 0)"),
            ("white-space", "nowrap"),
            ("word-wrap", "normal"),
        ]),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
