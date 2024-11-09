use std::rc::Rc;

use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    window, HtmlImageElement,
};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
struct AvatarContextValue {
    image_loading_status: ImageLoadingStatus,
    on_image_loading_status_change: Callback<ImageLoadingStatus>,
}

#[derive(PartialEq, Properties)]
pub struct AvatarProps<ChildProps: Clone + Default + PartialEq + SetAvatarChildProps> {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ChildProps, Html>>,
    #[prop_or_default]
    pub as_child_props: Option<ChildProps>,
    #[prop_or_default]
    pub children: Html,
}

pub trait SetAvatarChildProps {
    fn set_avatar_child_props(&mut self, props: AvatarChildProps);
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "span")]
pub struct AvatarChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,
}

impl SetAvatarChildProps for AvatarChildProps {
    fn set_avatar_child_props(&mut self, _: AvatarChildProps) {}
}

#[function_component]
pub fn Avatar<ChildProps: Clone + Default + PartialEq + SetAvatarChildProps = AvatarChildProps>(
    props: &AvatarProps<ChildProps>,
) -> Html {
    let image_loading_status = use_state_eq(|| ImageLoadingStatus::Idle);

    let on_image_loading_status_change = use_callback((), {
        let image_loading_status = image_loading_status.clone();

        move |value: ImageLoadingStatus, _| image_loading_status.set(value)
    });

    let context_value = use_memo(
        (image_loading_status, on_image_loading_status_change),
        |(image_loading_status, on_image_loading_status_change)| AvatarContextValue {
            image_loading_status: **image_loading_status,
            on_image_loading_status_change: on_image_loading_status_change.clone(),
        },
    );

    let child_props = AvatarChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    html! {
        <ContextProvider<AvatarContextValue> context={(*context_value).clone()}>
            if let Some(as_child) = props.as_child.as_ref() {
                {{
                    let mut as_child_props = props.as_child_props.clone().unwrap_or_default();
                    as_child_props.set_avatar_child_props(child_props);

                    as_child.emit(as_child_props)
                }}
            } else {
                {child_props.render(props.children.clone())}
            }
        </ContextProvider<AvatarContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct AvatarImageProps {
    #[prop_or_default]
    pub on_loading_status_change: Callback<ImageLoadingStatus>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    // Attributes from `img`
    #[prop_or_default]
    pub alt: Option<String>,
    #[prop_or_default]
    pub crossorigin: Option<String>,
    #[prop_or_default]
    pub decoding: Option<String>,
    #[prop_or_default]
    pub fetchpriority: Option<String>,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub ismap: bool,
    #[prop_or_default]
    pub loading: Option<String>,
    #[prop_or_default]
    pub referrerpolicy: Option<String>,
    #[prop_or_default]
    pub sizes: Option<String>,
    #[prop_or_default]
    pub src: Option<String>,
    #[prop_or_default]
    pub srcset: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_default]
    pub usemap: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<AvatarImageChildProps, Html>>,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "img", no_children = true)]
pub struct AvatarImageChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,

    // Attributes from `img`
    pub alt: Option<String>,
    pub crossorigin: Option<String>,
    pub decoding: Option<String>,
    pub fetchpriority: Option<String>,
    pub height: Option<String>,
    pub ismap: bool,
    pub loading: Option<String>,
    pub referrerpolicy: Option<String>,
    pub sizes: Option<String>,
    pub src: Option<String>,
    pub srcset: Option<String>,
    pub width: Option<String>,
    pub usemap: Option<String>,
}

#[function_component]
pub fn AvatarImage(props: &AvatarImageProps) -> Html {
    let context = use_context::<AvatarContextValue>().expect("Avatar context required.");
    let image_loading_status =
        use_image_loading_status(props.src.clone(), props.referrerpolicy.clone());
    let handle_loading_status_change = use_callback(
        props.on_loading_status_change.clone(),
        move |status: ImageLoadingStatus, on_loading_status_change| {
            on_loading_status_change.emit(status);
            context.on_image_loading_status_change.emit(status);
        },
    );

    use_effect_with(image_loading_status.clone(), move |image_loading_status| {
        if **image_loading_status != ImageLoadingStatus::Idle {
            handle_loading_status_change.emit(**image_loading_status);
        }
    });

    let child_props = AvatarImageChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone(),

        // Attributes from `img`
        alt: props.alt.clone(),
        crossorigin: props.crossorigin.clone(),
        decoding: props.decoding.clone(),
        fetchpriority: props.fetchpriority.clone(),
        height: props.height.clone(),
        ismap: props.ismap,
        loading: props.loading.clone(),
        referrerpolicy: props.referrerpolicy.clone(),
        sizes: props.sizes.clone(),
        src: props.src.clone(),
        srcset: props.srcset.clone(),
        width: props.width.clone(),
        usemap: props.usemap.clone(),
    };

    if *image_loading_status == ImageLoadingStatus::Loaded {
        if let Some(as_child) = props.as_child.as_ref() {
            as_child.emit(child_props)
        } else {
            child_props.render()
        }
    } else {
        Html::default()
    }
}

#[derive(PartialEq, Properties)]
pub struct AvatarFallbackProps {
    #[prop_or_default]
    pub delay_ms: Option<i32>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<AvatarFallbackChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "span")]
pub struct AvatarFallbackChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,
}

#[function_component]
pub fn AvatarFallback(props: &AvatarFallbackProps) -> Html {
    let context = use_context::<AvatarContextValue>().expect("Avatar context required.");
    let can_render = use_state_eq(|| props.delay_ms.is_none());

    use_effect_with(props.delay_ms, {
        let can_render = can_render.clone();

        move |delay_ms| {
            let mut cleanup: Option<Box<dyn FnOnce()>> = None;

            if let Some(delay_ms) = delay_ms {
                let handler: Closure<dyn Fn()> = Closure::new({
                    let can_render = can_render.clone();

                    move || {
                        can_render.set(true);
                    }
                });

                let timer_id = window()
                    .expect("Window should exist.")
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        handler.as_ref().unchecked_ref(),
                        *delay_ms,
                    )
                    .expect("Timeout should be set.");

                cleanup = Some(Box::new(move || {
                    window()
                        .expect("Window should exist.")
                        .clear_timeout_with_handle(timer_id);

                    // Move closure to prevent it from being dropped too early.
                    drop(handler);
                }));
            }

            move || {
                if let Some(cleanup) = cleanup {
                    cleanup();
                }
            }
        }
    });

    let child_props = AvatarFallbackChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    if *can_render && context.image_loading_status != ImageLoadingStatus::Loaded {
        if let Some(as_child) = props.as_child.as_ref() {
            as_child.emit(child_props)
        } else {
            child_props.render(props.children.clone())
        }
    } else {
        Html::default()
    }
}

#[hook]
fn use_image_loading_status(
    src: Option<String>,
    referrer_policy: Option<String>,
) -> UseStateHandle<ImageLoadingStatus> {
    let loading_status = use_state_eq(|| ImageLoadingStatus::Idle);
    let is_mounted = use_mut_ref(|| false);

    let update_status_loaded: Rc<Closure<dyn Fn()>> = use_ref(|| {
        Closure::new({
            let loading_status = loading_status.clone();
            let is_mounted = is_mounted.clone();

            move || {
                if *is_mounted.borrow() {
                    loading_status.set(ImageLoadingStatus::Loaded);
                }
            }
        })
    });
    let update_status_error: Rc<Closure<dyn Fn()>> = use_ref(|| {
        Closure::new({
            let loading_status = loading_status.clone();
            let is_mounted = is_mounted.clone();

            move || {
                if *is_mounted.borrow() {
                    loading_status.set(ImageLoadingStatus::Error);
                }
            }
        })
    });
    use_effect_with((), {
        let update_status_loaded = update_status_loaded.clone();
        let update_status_error = update_status_error.clone();

        move |_| {
            move || {
                // Move closures to prevent them from being dropped too early.
                drop(update_status_loaded);
                drop(update_status_error);
            }
        }
    });

    use_effect_with((src, referrer_policy), {
        let loading_status = loading_status.clone();

        move |(src, referrer_policy)| {
            if let Some(src) = &src {
                *is_mounted.borrow_mut() = true;

                let image = window()
                    .expect("Window should exist.")
                    .document()
                    .expect("Document should exist.")
                    .create_element("img")
                    .map(|element| element.unchecked_into::<HtmlImageElement>())
                    .expect("Image element should be created.");

                loading_status.set(ImageLoadingStatus::Loading);

                image
                    .add_event_listener_with_callback(
                        "load",
                        (*update_status_loaded).as_ref().unchecked_ref(),
                    )
                    .expect("Load event listener should be added.");
                image
                    .add_event_listener_with_callback(
                        "error",
                        (*update_status_error).as_ref().unchecked_ref(),
                    )
                    .expect("Error event listener should be added.");
                image.set_src(src);
                if let Some(referrer_policy) = referrer_policy {
                    image.set_referrer_policy(referrer_policy);
                }
            } else {
                loading_status.set(ImageLoadingStatus::Error);
            }

            move || {
                *is_mounted.borrow_mut() = false;
            }
        }
    });

    loading_status
}
