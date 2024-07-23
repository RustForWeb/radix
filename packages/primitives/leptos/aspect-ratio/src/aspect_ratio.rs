use leptos::{html::AnyElement, *};
use radix_leptos_primitive::Primitive;

#[component]
pub fn AspectRatio(
    #[prop(into, optional)] ratio: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let ratio = Signal::derive(move || ratio.get().unwrap_or(1.0));

    let mut attrs = attrs.clone();
    // TODO: merge existing style
    attrs.extend([(
        "style",
        // Ensures children expand in ratio
        "position: absolute; top: 0px; right: 0px; bottom: 0px; left: 0px;".into_attribute(),
    )]);

    view! {
        <div
            // Ensures inner element is contained
            style:position="relative"
            // Ensures padding bottom trick maths works
            style:width="100%"
            style:padding-bottom=move || format!("{}%", 100.0 / ratio.get())
            data-radix-aspect-ratio-wrapper=""
        >
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attrs=attrs
            >
                {children()}
            </Primitive>
        </div>
    }
}
