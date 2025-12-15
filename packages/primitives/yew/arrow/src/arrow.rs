use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct ArrowProps<ChildProps: Clone + Default + PartialEq + SetArrowChildProps> {
    #[prop_or(10.0)]
    pub width: f64,
    #[prop_or(5.0)]
    pub height: f64,

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
    pub as_child: Option<Callback<ChildProps, Html>>,
    #[prop_or_default]
    pub as_child_props: Option<ChildProps>,
}

pub trait SetArrowChildProps {
    fn set_arrow_child_props(&mut self, props: ArrowChildProps);
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "svg")]
#[expect(non_snake_case)]
pub struct ArrowChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `svg`
    pub width: String,
    pub height: String,
    pub viewBox: String,
    pub preserveAspectRatio: String,
}

impl SetArrowChildProps for ArrowChildProps {
    fn set_arrow_child_props(&mut self, _: ArrowChildProps) {}
}

#[function_component]
pub fn Arrow<ChildProps: Clone + Default + PartialEq + SetArrowChildProps = ArrowChildProps>(
    props: &ArrowProps<ChildProps>,
) -> Html {
    let child_props = ArrowChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),

        // Attributes from `svg`
        width: props.width.to_string(),
        height: props.height.to_string(),
        viewBox: "0 0 30 10".to_owned(),
        preserveAspectRatio: "none".to_owned(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        let mut as_child_props = props.as_child_props.clone().unwrap_or_default();
        as_child_props.set_arrow_child_props(child_props);

        as_child.emit(as_child_props)
    } else {
        child_props.render(html! {
            <polygon points="0,0 30,0 15,10" />
        })
    }
}
