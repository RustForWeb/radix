use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::Primitive;

#[component]
pub fn VisuallyHidden(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            children=children

            // See: https://github.com/twbs/bootstrap/blob/main/scss/mixins/_visually-hidden.scss
            style:position="absolute"
            style:border="0px"
            style:width="1px"
            style:height="1px"
            style:padding="0px"
            style:margin="-1px"
            style:overflow="hidden"
            style:clip="rect(0, 0, 0, 0)"
            style:white-space="nowrap"
            style:word-wrap="normal"
        />
    }
}
