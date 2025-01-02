use leptos::prelude::*;
use leptos::context::Provider;
use leptos::{html};
use leptos::html::Img;
use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::JsCast;
use leptos_node_ref::prelude::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
use leptos_maybe_callback::MaybeCallback;
use radix_leptos_context::create_context;
use radix_leptos_primitive::{Primitive, VoidPrimitive};

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Clone, Debug)]
struct AvatarContextValue {
    image_loading_status: ReadSignal<ImageLoadingStatus>,
    on_image_loading_status_change: Callback<ImageLoadingStatus>,
}

/* -------------------------------------------------------------------------------------------------
 * Avatar (Root)
 * -----------------------------------------------------------------------------------------------*/

const AVATAR_NAME: &'static str = "Avatar";

create_context!(
    context_type: AvatarContextValue,
    provider: AvatarProvider,
    hook: use_avatar_context,
    root: AVATAR_NAME
);

#[component]
#[allow(non_snake_case)]
pub fn Avatar(
    /// If `true`, renders only its children without a `<span>` wrapper.
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    /// A reference to the underlying `<span>` element, if needed.
    #[prop(into, optional)] node_ref: AnyNodeRef,
    /// The children of the Avatar component.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    // Initialize the image loading status signal using `RwSignal`
    let image_loading_status = RwSignal::new(ImageLoadingStatus::Idle);

    // Define the context value with the current loading status and a callback to update it
    let context_value = AvatarContextValue {
        image_loading_status: image_loading_status.read_only(),
        on_image_loading_status_change: Callback::new(move |status| {
            image_loading_status.set(status);
        }),
    };

    view! {
        <AvatarProvider value=context_value>
            <Primitive element=leptos::html::span as_child=as_child node_ref=node_ref>
                {children.with_value(|children| children())}
            </Primitive>
        </AvatarProvider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AvatarImage
 * -----------------------------------------------------------------------------------------------*/

const IMAGE_NAME: &'static str = "AvatarImage";

#[component]
#[allow(non_snake_case)]
pub fn AvatarImage(
    #[prop(into, optional)] src: MaybeProp<String>,
    #[prop(into, optional)] referrer_policy: MaybeProp<String>,
    #[prop(into, optional)] on_loading_status_change: MaybeCallback<ImageLoadingStatus>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<Img>,
) -> impl IntoView {
    let context = use_avatar_context(IMAGE_NAME);
    let loading_status = use_image_loading_status(src.clone(), referrer_policy.clone());

    // Update context and callback when loading status changes
    Effect::new(move |_| {
        let status = loading_status.get();
        context.on_image_loading_status_change.run(status);
        on_loading_status_change.run(status);
    });

    view! {
        <Show
            when=move || loading_status.get() == ImageLoadingStatus::Loaded
            fallback=|| ()
        >
            <VoidPrimitive
                element=html::img
                as_child=as_child
                node_ref=node_ref.into_any()
                attr:src=move || src.get()
                attr:referrerpolicy=move || referrer_policy.get()
            >
                {()}
            </VoidPrimitive>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AvatarFallback
 * -----------------------------------------------------------------------------------------------*/

const FALLBACK_NAME: &'static str = "AvatarFallback";

#[component]
pub fn AvatarFallback(
    /// Children (for example, initials or an icon).
    children: TypedChildrenFn<impl IntoView + 'static>,
    /// Delay (in ms) before showing the fallback `<span>`. If no delay, fallback appears immediately.
    #[prop(into, optional)] delay_ms: MaybeProp<f64>,
    /// If `true`, renders only its children without a `<span>` wrapper.
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    /// A reference to the `<span>` element for the fallback.
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = use_avatar_context(FALLBACK_NAME);

    // use_timeout_fn from leptos_use to handle the delay before showing fallback
    let UseTimeoutFnReturn { start, stop, is_pending, .. } = use_timeout_fn(
        move |_| {},
        delay_ms.get().unwrap_or_default(),
    );

    // If no delay is set, fallback can render immediately
    let can_render = RwSignal::new(delay_ms.get().is_none());

    // Re-initialize the timer whenever `delay_ms` changes
    Effect::new(move || {
        stop();
        can_render.set(delay_ms.get().is_none());

        #[cfg(debug_assertions)]
        leptos::logging::log!(
            "[{FALLBACK_NAME}] delay_ms changed: {:?}",
            delay_ms.get()
        );

        if let Some(ms) = delay_ms.get() {
            #[cfg(debug_assertions)]
            leptos::logging::log!("[{FALLBACK_NAME}] Starting timeout for {} ms", ms);
            start(ms as i32);
        }
    });

    // Watch if the timer has completed
    Effect::new(move || {
        if !is_pending.get() && delay_ms.get().is_some() {
            #[cfg(debug_assertions)]
            leptos::logging::log!("[{FALLBACK_NAME}] Timer completed, can_render=true");
            can_render.set(true);
        }
    });

    // Render fallback <span> only if `can_render` is true and the image is not loaded
    view! {
        <Show
            when=move || {
                can_render.get() && context.image_loading_status.get() != ImageLoadingStatus::Loaded
            }
            fallback=|| ()
        >
            <Primitive element=html::span as_child=as_child node_ref=node_ref>
                {children.with_value(|children| children())}
            </Primitive>
        </Show>
    }
}

/* -----------------------------------------------------------------------------------------------*/

fn use_image_loading_status(
    src: MaybeProp<String>,
    referrer_policy: MaybeProp<String>,
) -> ReadSignal<ImageLoadingStatus> {
    let loading_status = RwSignal::new(ImageLoadingStatus::Idle);

    Effect::new(move |_| {
        if let Some(src_val) = src.get() {
            #[cfg(debug_assertions)]
            leptos::logging::log!("[{IMAGE_NAME}] Starting load for: {}", src_val);

            loading_status.set(ImageLoadingStatus::Loading);

            let image = web_sys::HtmlImageElement::new().unwrap();

            // Clone image for closures
            let image_clone = image.clone();
            let onload = Closure::wrap(Box::new(move || {
                if image_clone.natural_width() > 0 {
                    #[cfg(debug_assertions)]
                    leptos::logging::log!("[{IMAGE_NAME}] Load successful");
                    loading_status.set(ImageLoadingStatus::Loaded);
                } else {
                    #[cfg(debug_assertions)]
                    leptos::logging::log!("[{IMAGE_NAME}] Load failed - invalid image");
                    loading_status.set(ImageLoadingStatus::Error);
                }
            }) as Box<dyn FnMut()>);

            let onerror = Closure::wrap(Box::new(move || {
                #[cfg(debug_assertions)]
                leptos::logging::log!("[{IMAGE_NAME}] Load failed");
                loading_status.set(ImageLoadingStatus::Error);
            }) as Box<dyn FnMut()>);

            image.set_onload(Some(onload.as_ref().unchecked_ref()));
            image.set_onerror(Some(onerror.as_ref().unchecked_ref()));

            if let Some(policy) = referrer_policy.get() {
                image.set_referrer_policy(&policy);
            }

            image.set_src(&src_val);

            onload.forget();
            onerror.forget();
        } else {
            #[cfg(debug_assertions)]
            leptos::logging::log!("[{IMAGE_NAME}] No src provided");
            loading_status.set(ImageLoadingStatus::Error);
        }
    });

    loading_status.read_only()
}

/* -------------------------------------------------------------------------------------------------
 * Primitive re-exports
 * -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    // Re-export core items so consumers can use avatar::primitive::* as AvatarPrimitive
    pub use super::*;
    pub use Avatar as Root;
    pub use AvatarImage as Image;
    pub use AvatarFallback as Fallback;
}
