use std::fmt::{Display, Formatter};

use leptos::{html, prelude::*};
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

#[component]
pub fn Separator(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
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
                if decorative.get() {
                    None
                } else {
                    match orientation.get() {
                        Orientation::Vertical => Some("vertical".to_owned()),
                        Orientation::Horizontal => None,
                    }
                }
            }
            attr:role=move || if decorative.get() { "none" } else { "separator" }
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}
