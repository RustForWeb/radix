use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::Primitive;

/* -------------------------------------------------------------------------------------------------
 * VisuallyHidden
 * -----------------------------------------------------------------------------------------------*/

/// A component that visually hides its children while keeping them accessible
/// to screen reader users. Matches the React `VisuallyHidden` component’s
/// default styles and behavior.
#[component]
#[allow(non_snake_case)]
pub fn VisuallyHidden(
    /// The content to be visually hidden but still accessible to screen readers.
    children: TypedChildrenFn<impl IntoView + 'static>,

    /// If `true`, the `Primitive` is rendered as the child element rather than wrapped.
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,

    /// A reference to the underlying DOM node.
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
) -> impl IntoView {
    // See: https://github.com/twbs/bootstrap/blob/main/scss/mixins/_visually-hidden.scss
    view! {
        <Primitive
            element=html::span
            children=children
            as_child=as_child
            node_ref=node_ref

            // visually hide this content but keep it available to assistive tech
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

/* -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    pub use super::*;
    pub use VisuallyHidden as Root;
}
