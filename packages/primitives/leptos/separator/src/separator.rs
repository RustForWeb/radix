use std::fmt::{Display, Formatter};

use leptos::{html::AnyElement, *};
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

impl IntoAttribute for Orientation {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.to_string().into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

#[component]
pub fn Separator(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let orientation = Signal::derive(move || orientation.get().unwrap_or_default());
    let aria_orientation = Signal::derive(move || match orientation.get() {
        Orientation::Horizontal => None,
        Orientation::Vertical => Some("vertical".to_string()),
    });
    let decorative = Signal::derive(move || decorative.get().unwrap_or_default());

    let mut attrs = attrs.clone();
    attrs.extend([
        ("data-orientation", orientation.into_attribute()),
        (
            "aria-orientation",
            (match decorative.get() {
                true => aria_orientation.get(),
                false => None,
            })
            .into_attribute(),
        ),
        (
            "role",
            (match decorative.get() {
                true => "none",
                false => "separator",
            })
            .into_attribute(),
        ),
    ]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}
