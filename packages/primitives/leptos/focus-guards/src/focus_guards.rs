use std::sync::atomic::{AtomicU64, Ordering};

use leptos::prelude::*;
use web_sys::{wasm_bindgen::JsCast, Element, HtmlElement};

static COUNT: AtomicU64 = AtomicU64::new(0);

#[component]
pub fn FocusGuards(children: ChildrenFn) -> impl IntoView {
    use_focus_guards();

    view! {
        {children()}
    }
}

pub fn use_focus_guards() {
    Effect::new(move |_| {
        let edge_guards = document()
            .query_selector_all("[data-radix-focus-guard]")
            .expect("Document should be queried.");

        let body = document().body().expect("Document should have body.");
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
    });

    on_cleanup(move || {
        if COUNT.load(Ordering::Relaxed) == 1 {
            let node_list = document()
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
    });
}

fn create_focus_guard() -> HtmlElement {
    let element = document()
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
