use leptos::{prelude::*, html, attribute_interceptor::AttributeInterceptor};
use radix_leptos_primitive::Primitive;
use leptos_node_ref::AnyNodeRef;

const DEFAULT_RATIO: f64 = 1.0;

/* -------------------------------------------------------------------------------------------------
 * AspectRatio
 * -----------------------------------------------------------------------------------------------*/

const NAME: &'static str = "AspectRatio";

#[component]
#[allow(non_snake_case)]
pub fn AspectRatio(
    /// Children passed to the AspectRatio component
    children: TypedChildrenFn<impl IntoView + 'static>,
    /// Change the default rendered element for the one passed as a child
    #[prop(into, optional, default = false.into())]
    as_child: MaybeProp<bool>,
    /// The desired ratio when rendering the content (e.g., 16/9). Defaults to 1.0 if not specified.
    #[prop(into, optional, default = DEFAULT_RATIO.into())]
    ratio: MaybeProp<f64>,
    /// Reference to the underlying DOM node
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
) -> impl IntoView {
    // attribute interceptor incurs this requirement
    let children = StoredValue::new(children.into_inner());

    // calculates the percent-based padding for the aspect ratio
    let padding_bottom = Signal::derive(move || {
        100.0
            / ratio
            .get()
            .unwrap_or(DEFAULT_RATIO)
            .clamp(f64::EPSILON, f64::MAX)
    });

    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        leptos::logging::log!("[{NAME}] ratio: {:?}", ratio.get());
        leptos::logging::log!("[{NAME}] as_child: {:?}", as_child.get());
        leptos::logging::log!("[{NAME}] node_ref: {:?}", node_ref.get());
        leptos::logging::log!("[{NAME}] padding_bottom: {:?}", padding_bottom.get());
    });

    view! {
        // replicate the radix react spread props
        <AttributeInterceptor let:attrs>
            // ensures inner element is contained
            <div
                style:position="relative"
                // ensures padding bottom trick works
                style:width="100%"
                style:padding-bottom=move || format!("{}%", padding_bottom.get())
                data-radix-aspect-ratio-wrapper=""
            >
                <Primitive
                    // ensures children expand to fill the ratio
                    element={html::div}
                    as_child={as_child}
                    node_ref={node_ref}
                    {..attrs}
                    style:position="absolute"
                    style:top="0"
                    style:right="0"
                    style:bottom="0"
                    style:left="0"
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </div>
        </AttributeInterceptor>
    }
}

/* -----------------------------------------------------------------------------------------------*/

pub use AspectRatio as Root;
