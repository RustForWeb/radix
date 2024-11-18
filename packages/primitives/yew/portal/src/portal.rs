use web_sys::{wasm_bindgen::JsCast, window};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct PortalProps {
    /// An optional container where the portaled content should be appended.
    #[prop_or_default]
    pub container: Option<web_sys::Element>,
    /// An optional container where the portaled content should be appended.
    #[prop_or_default]
    pub container_ref: Option<NodeRef>,

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
    pub as_child: Option<Callback<PortalChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct PortalChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Portal(props: &PortalProps) -> Html {
    let container = use_state_eq(|| None);
    use_effect({
        let container_ref = props.container_ref.clone();
        let container = container.clone();

        move || {
            if let Some(container_ref) = &container_ref {
                container.set(container_ref.cast::<web_sys::Element>());
            }
        }
    });

    let container = (*container)
        .clone()
        .or_else(|| props.container.clone())
        .or_else(|| {
            window()
                .and_then(|window| window.document())
                .and_then(|document| document.body())
                .and_then(|body| body.dyn_into::<web_sys::Element>().ok())
        });

    let child_props = PortalChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    html! {
        if let Some(container) = container {
            {create_portal(
                if let Some(as_child) = props.as_child.as_ref() {
                    as_child.emit(child_props)
                } else {
                    child_props.render(props.children.clone())
                },
                container,
            )}
        }
    }
}
