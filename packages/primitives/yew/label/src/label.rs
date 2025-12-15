use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct LabelProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `label`
    #[prop_or_default]
    pub r#for: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_mouse_down: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<LabelChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "label")]
pub struct LabelChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `label`
    pub r#for: Option<String>,

    // Event handler attributes
    pub onmousedown: Callback<MouseEvent>,
}

#[function_component]
pub fn Label(props: &LabelProps) -> Html {
    let onmousedown = use_callback(
        props.on_mouse_down.clone(),
        |event: MouseEvent, on_mouse_down| {
            let target = event
                .target()
                .expect("Target should exist.")
                .dyn_into::<web_sys::Element>()
                .expect("Target should be an Element.");

            // Only prevent text selection if clicking inside the label itself
            if target
                .closest("button, input, select, textarea")
                .expect("Element should be able to query closest.")
                .is_some()
            {
                return;
            }

            on_mouse_down.emit(event.clone());

            // Prevent text selection when double clicking label
            if !event.default_prevented() && event.detail() > 1 {
                event.prevent_default();
            }
        },
    );

    let child_props = LabelChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone(),

        // Attributes from `label`
        r#for: props.r#for.clone(),

        // Event handler attributes
        onmousedown,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
