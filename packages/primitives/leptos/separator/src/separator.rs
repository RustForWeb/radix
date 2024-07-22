use leptos::{html::AnyElement, *};
use radix_leptos_primitive::Primitive;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl From<Orientation> for String {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Horizontal => "horizontal".into(),
            Orientation::Vertical => "vertical".into(),
        }
    }
}

impl IntoAttribute for Orientation {
    fn into_attribute(self) -> Attribute {
        let s: String = self.into();
        Attribute::String(s.into())
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
