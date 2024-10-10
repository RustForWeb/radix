use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LabelAsChildProps {
    pub node_ref: NodeRef,
    pub onmousedown: Callback<MouseEvent>,
}

#[derive(PartialEq, Properties)]
pub struct LabelProps {
    #[prop_or_default]
    pub r#for: Option<String>,
    #[prop_or_default]
    pub on_mouse_down: Callback<MouseEvent>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<LabelAsChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
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

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(LabelAsChildProps {
            node_ref: props.node_ref.clone(),
            onmousedown,
        })
    } else {
        html! {
            <label
                ref={props.node_ref.clone()}
                id={props.id.clone()}
                class={props.class.clone()}
                style={props.style.clone()}
                for={props.r#for.clone()}
                onmousedown={onmousedown}
            >
                {props.children.clone()}
            </label>
        }
    }
}
