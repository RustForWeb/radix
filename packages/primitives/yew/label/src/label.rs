use radix_yew_primitive::Primitive;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_attrs::{attrs, Attrs};

#[derive(PartialEq, Properties)]
pub struct LabelProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Label(props: &LabelProps) -> Html {
    let onmousedown = use_callback((), |event: MouseEvent, _| {
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

        // TODO?
        // if let Some(on_mouse_down) = on_mouse_down {
        //     Callable::call(&on_mouse_down, event.clone());
        // }

        // Prevent text selection when double clicking label
        if !event.default_prevented() && event.detail() > 1 {
            event.prevent_default();
        }
    });

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! { onmousedown={onmousedown} })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="label"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}
