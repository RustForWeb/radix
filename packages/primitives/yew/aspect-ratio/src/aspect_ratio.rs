use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};

#[derive(PartialEq, Properties)]
pub struct AspectRatioProps {
    #[prop_or(1.0)]
    pub ratio: f64,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<AspectRatioChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct AspectRatioChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: String,
}

#[function_component]
pub fn AspectRatio(props: &AspectRatioProps) -> Html {
    let child_props = AspectRatioChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: format!(
            // Ensures children expand in ratio.
            "position: absolute; top: 0px; right: 0px; bottom: 0px; left: 0px;{}",
            props.style.clone().unwrap_or_default()
        ),
    };

    html! {
        <div
            // Posittion ensures inner element is contained.
            // Width ensures padding bottom trick maths works.
            style={format!("position: relative; width: 100%; padding-bottom: {}%;", 100.0 / props.ratio)}
            data-radix-aspect-ratio-wrapper=""
        >
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        </div>
    }
}
