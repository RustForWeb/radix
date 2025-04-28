use std::{ops::Deref, rc::Rc};

use dioxus::prelude::*;
use dioxus::web::WebEventExt;

#[derive(Clone, Copy)]
pub struct NodeRef {
    signal: Signal<Option<Rc<MountedData>>>,
}

impl NodeRef {
    pub fn onmounted(mut self) -> impl FnMut(Event<MountedData>) {
        move |event| {
            self.signal.set(Some(event.data()));
        }
    }

    pub fn web_element(&self) -> Option<web_sys::Element> {
        (self.signal)().map(|mounted_data| mounted_data.as_web_event().clone())
    }
}

impl Deref for NodeRef {
    type Target = Signal<Option<Rc<MountedData>>>;

    fn deref(&self) -> &Self::Target {
        &self.signal
    }
}

pub fn use_node_ref() -> NodeRef {
    NodeRef {
        signal: use_signal(|| None),
    }
}

#[component]
pub fn UseNodeRef() -> Element {
    let node_ref = use_node_ref();

    use_effect(move || {
        if let Some(element) = node_ref.web_element() {
            log::info!("mounted {element:?}");
        }
    });

    rsx! {
        div {
            onmounted: node_ref.onmounted(),
            "Test"
        }
    }
}
