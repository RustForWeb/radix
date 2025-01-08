use std::fmt::{Display, Formatter};
use leptos::prelude::*;
use leptos::html;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::Primitive;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
        )
    }
}

impl From<&str> for Orientation {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "vertical" => Orientation::Vertical,
            _ => Orientation::Horizontal,
        }
    }
}

impl From<String> for Orientation {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Separator(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: MaybeProp<String>,
    // if as_child=true, there will be no attribute passthrough to the children,
    // but we opt for this design so we can allow children to be optional.
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let orientation = Signal::derive(move || orientation.get().unwrap_or_default());
    let decorative = Signal::derive(move || decorative.get().unwrap_or_default());
    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:data-orientation=move || orientation.get().to_string()
            attr:aria-orientation=move || {
                if !decorative.get() {
                    match orientation.get() {
                        Orientation::Vertical => Some("vertical".to_string()),
                        Orientation::Horizontal => None,
                    }
                } else {
                    None
                }
            }
            attr:role=move || if decorative.get() { "none" } else { "separator" }
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}
