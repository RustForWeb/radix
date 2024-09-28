use std::sync::atomic::{AtomicU64, Ordering};

use web_sys::{wasm_bindgen::JsCast, window, Element, HtmlElement};
use yew::prelude::*;

static COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(PartialEq, Properties)]
pub struct FocusGuardsProps {
    #[prop_or_default]
    children: Html,
}

#[function_component]
pub fn FocusGuards(props: &FocusGuardsProps) -> Html {
    use_focus_guards();

    props.children.clone()
}

#[hook]
pub fn use_focus_guards() {
    use_effect(|| {
        let document = window()
            .expect("Window should exist.")
            .document()
            .expect("Document should eixst.");

        let edge_guards = document
            .query_selector_all("[data-radix-focus-guard]")
            .expect("Document should be queried.");

        let body = document.body().expect("Document should have body.");
        body.insert_adjacent_element(
            "afterbegin",
            &edge_guards
                .get(0)
                .map(|node| node.unchecked_into::<Element>())
                .unwrap_or(create_focus_guard().into()),
        )
        .expect("Element should be inserted.");
        body.insert_adjacent_element(
            "beforeend",
            &edge_guards
                .get(1)
                .map(|node| node.unchecked_into::<Element>())
                .unwrap_or(create_focus_guard().into()),
        )
        .expect("Element should be inserted.");

        COUNT.fetch_add(1, Ordering::Relaxed);

        move || {
            if COUNT.load(Ordering::Relaxed) == 1 {
                let node_list = document
                    .query_selector_all("[data-radix-focus-guard]")
                    .expect("Document should be queried.");
                for n in 0..node_list.length() {
                    if let Some(element) = node_list
                        .get(n)
                        .map(|node| node.unchecked_into::<Element>())
                    {
                        element.remove();
                    }
                }
            }
            COUNT.fetch_sub(1, Ordering::Relaxed);
        }
    });
}

fn create_focus_guard() -> HtmlElement {
    let element = window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
        .create_element("span")
        .expect("Element should be created.")
        .unchecked_into::<HtmlElement>();
    element
        .set_attribute("data-radix-focus-guard", "")
        .expect("Attribute should be set.");
    element.set_tab_index(0);
    element
        .style()
        .set_css_text("outline: none; opacity: 0; position: fixed; pointer-events: none;");

    element
}
