use leptos::{
    html::AnyElement,
    web_sys::{
        wasm_bindgen::{closure::Closure, JsCast},
        HtmlImageElement,
    },
    *,
};
use radix_leptos_primitive::Primitive;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[component]
pub fn Avatar(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let (image_loading_status, set_image_loading_status) = create_signal(ImageLoadingStatus::Idle);

    let context_value = AvatarContextValue {
        image_loading_status,
        on_image_loading_status_change: Callback::new(move |image_loading_status| {
            set_image_loading_status.set(image_loading_status)
        }),
    };

    view! {
        <Provider value=context_value>
             <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attrs=attrs
            >
                {children()}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into, optional)] src: MaybeProp<String>,
    #[prop(into, optional)] on_loading_status_change: Option<Callback<ImageLoadingStatus>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<AvatarContextValue>();
    let image_loading_status = use_image_loading_status(src.clone());
    let handle_loading_status_change = move |status: ImageLoadingStatus| {
        if let Some(on_loading_status_change) = on_loading_status_change {
            on_loading_status_change.call(status);
        }
        context.on_image_loading_status_change.call(status);
    };

    Effect::new(move |_| {
        let image_loading_status = image_loading_status.get();
        if image_loading_status != ImageLoadingStatus::Idle {
            handle_loading_status_change(image_loading_status);
        }
    });

    let mut attrs = attrs.clone();
    attrs.extend([("src", src.into_attribute())]);
    let attrs = StoredValue::new(attrs);

    view! {
        <Show when=move || image_loading_status.get() == ImageLoadingStatus::Loaded>
            <Primitive
                element=html::img
                as_child=as_child
                node_ref=node_ref
                attrs=attrs.get_value()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Show>
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(into, optional)] delay_ms: MaybeProp<i32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    let context = expect_context::<AvatarContextValue>();
    let (can_render, set_can_render) = create_signal(delay_ms.get().is_none());

    let handler: Closure<dyn Fn()> = Closure::new(move || {
        set_can_render.set(true);
    });

    let timer_id = StoredValue::new(None::<i32>);
    Effect::new(move |_| {
        if let Some(timer_id) = timer_id.get_value() {
            window().clear_timeout_with_handle(timer_id);
        }

        if let Some(delay_ms) = delay_ms.get() {
            timer_id.set_value(Some(
                window()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        handler.as_ref().unchecked_ref(),
                        delay_ms,
                    )
                    .expect("Timeout should be set."),
            ));
        }
    });

    on_cleanup(move || {
        if let Some(timer_id) = timer_id.get_value() {
            window().clear_timeout_with_handle(timer_id);
        }
    });

    view! {
        <Show when=move || can_render.get() && context.image_loading_status.get() != ImageLoadingStatus::Loaded>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attrs=attrs.get_value()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Show>
    }
}

fn use_image_loading_status(src: MaybeProp<String>) -> ReadSignal<ImageLoadingStatus> {
    let (loading_status, set_loading_status) = create_signal(ImageLoadingStatus::Idle);
    let is_mounted = StoredValue::new(true);

    let update_status_loaded: Closure<dyn Fn()> = Closure::new(move || {
        if is_mounted.get_value() {
            set_loading_status.set(ImageLoadingStatus::Loaded);
        }
    });
    let update_status_error: Closure<dyn Fn()> = Closure::new(move || {
        if is_mounted.get_value() {
            set_loading_status.set(ImageLoadingStatus::Error);
        }
    });

    Effect::new(move |_| {
        if let Some(src) = src.get() {
            let image = document()
                .create_element("img")
                .map(|element| element.unchecked_into::<HtmlImageElement>())
                .expect("Image element should be created.");

            set_loading_status.set(ImageLoadingStatus::Loading);

            image
                .add_event_listener_with_callback(
                    "load",
                    update_status_loaded.as_ref().unchecked_ref(),
                )
                .expect("Load event listener should be added.");
            image
                .add_event_listener_with_callback(
                    "error",
                    update_status_error.as_ref().unchecked_ref(),
                )
                .expect("Error event listener should be added.");
            image.set_src(&src);
        } else {
            set_loading_status.set(ImageLoadingStatus::Error);
        }
    });

    on_cleanup(move || {
        is_mounted.set_value(false);
    });

    loading_status
}
