// TODO: Remove
#![allow(dead_code)]

use radix_yew_direction::{Direction, use_direction};
use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

#[derive(Clone, Debug, PartialEq)]
pub struct Sizes {
    content: f64,
    viewport: f64,
    scrollbar: SizesScrollbar,
}

#[derive(Clone, Debug, PartialEq)]
struct SizesScrollbar {
    size: f64,
    padding_start: f64,
    padding_end: f64,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ScrollAreaType {
    Auto,
    Always,
    Scroll,
    #[default]
    Hover,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ScrollAreaOrientation {
    Horizontal,
    #[default]
    Vertical,
}

#[derive(Clone, PartialEq)]
struct ScrollAreaContextValue {
    r#type: ScrollAreaType,
    dir: Direction,
    scroll_hide_delay: u32,
    scroll_area_ref: NodeRef,
    viewport_ref: NodeRef,
    content_ref: NodeRef,
    scrollbar_x_ref: NodeRef,
    scrollbar_x_enabled: bool,
    on_scrollbar_x_enabled_change: Callback<bool>,
    scrollbar_y_ref: NodeRef,
    scrollbar_y_enabled: bool,
    on_scrollbar_y_enabled_change: Callback<bool>,
    on_corner_width_change: Callback<f64>,
    on_corner_height_change: Callback<f64>,
}

#[derive(PartialEq, Properties)]
pub struct ScrollAreaProps {
    #[prop_or_default]
    pub r#type: ScrollAreaType,
    #[prop_or_default]
    pub dir: Option<Direction>,
    #[prop_or(600)]
    pub scroll_hide_delay: u32,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct ScrollAreaChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub dir: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn ScrollArea(props: &ScrollAreaProps) -> Html {
    let scroll_area_ref = use_node_ref();
    let viewport_ref = use_node_ref();
    let content_ref = use_node_ref();
    let scrollbar_x_ref = use_node_ref();
    let scrollbar_y_ref = use_node_ref();
    let corner_width = use_state_eq(|| 0.0);
    let corner_height = use_state_eq(|| 0.0);
    let scrollbar_x_enabled = use_state_eq(|| false);
    let scrollbar_y_enabled = use_state_eq(|| false);
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), scroll_area_ref.clone()]);
    let direction = use_direction(props.dir);

    let handle_corner_width_change = use_callback(corner_width.clone(), |value, corner_width| {
        corner_width.set(value);
    });
    let handle_corner_height_change =
        use_callback(corner_height.clone(), |value, corner_height| {
            corner_height.set(value);
        });
    let handle_scrollbar_x_enabled_change =
        use_callback(scrollbar_x_enabled.clone(), |value, scrollbar_x_enabled| {
            scrollbar_x_enabled.set(value);
        });
    let handle_scrollbar_y_enabled_change =
        use_callback(scrollbar_y_enabled.clone(), |value, scrollbar_y_enabled| {
            scrollbar_y_enabled.set(value);
        });

    // Only tuples up to 12 elements implement PartialEq.
    #[derive(PartialEq)]
    struct MemoDeps {
        r#type: ScrollAreaType,
        scroll_hide_delay: u32,
        scroll_area_ref: NodeRef,
        viewport_ref: NodeRef,
        content_ref: NodeRef,
        scrollbar_x_ref: NodeRef,
        scrollbar_y_ref: NodeRef,
        scrollbar_x_enabled: UseStateHandle<bool>,
        scrollbar_y_enabled: UseStateHandle<bool>,
        direction: Direction,
        handle_corner_width_change: Callback<f64>,
        handle_corner_height_change: Callback<f64>,
        handle_scrollbar_x_enabled_change: Callback<bool>,
        handle_scrollbar_y_enabled_change: Callback<bool>,
    }

    let context_value = use_memo(
        MemoDeps {
            r#type: props.r#type,
            scroll_hide_delay: props.scroll_hide_delay,
            scroll_area_ref,
            viewport_ref,
            content_ref,
            scrollbar_x_ref,
            scrollbar_y_ref,
            scrollbar_x_enabled,
            scrollbar_y_enabled,
            direction,
            handle_corner_width_change,
            handle_corner_height_change,
            handle_scrollbar_x_enabled_change,
            handle_scrollbar_y_enabled_change,
        },
        |MemoDeps {
             r#type,
             scroll_hide_delay,
             scroll_area_ref,
             viewport_ref,
             content_ref,
             scrollbar_x_ref,
             scrollbar_y_ref,
             scrollbar_x_enabled,
             scrollbar_y_enabled,
             direction,
             handle_corner_width_change,
             handle_corner_height_change,
             handle_scrollbar_x_enabled_change,
             handle_scrollbar_y_enabled_change,
         }| ScrollAreaContextValue {
            r#type: *r#type,
            dir: *direction,
            scroll_hide_delay: *scroll_hide_delay,
            scroll_area_ref: scroll_area_ref.clone(),
            viewport_ref: viewport_ref.clone(),
            content_ref: content_ref.clone(),
            scrollbar_x_ref: scrollbar_x_ref.clone(),
            scrollbar_x_enabled: **scrollbar_x_enabled,
            on_scrollbar_x_enabled_change: handle_scrollbar_x_enabled_change.clone(),
            scrollbar_y_ref: scrollbar_y_ref.clone(),
            scrollbar_y_enabled: **scrollbar_y_enabled,
            on_scrollbar_y_enabled_change: handle_scrollbar_y_enabled_change.clone(),
            on_corner_width_change: handle_corner_width_change.clone(),
            on_corner_height_change: handle_corner_height_change.clone(),
        },
    );

    let child_props = ScrollAreaChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        dir: direction.to_string(),
        id: props.id.clone(),
        style: props.style.clone().with_defaults([
            ("position", "relative".to_owned()),
            // Pass corner sizes as CSS vars to reduce re-renders of context consumers.
            (
                "--radix-scroll-area-corner-width",
                format!("{}px", *corner_width),
            ),
            (
                "--radix-scroll-area-corner-height",
                format!("{}px", *corner_height),
            ),
        ]),
    };

    html! {
        <ContextProvider<ScrollAreaContextValue> context={(*context_value).clone()}>
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        </ContextProvider<ScrollAreaContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct ScrollAreaViewportProps {
    #[prop_or_default]
    pub nonce: Option<String>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaViewportChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct ScrollAreaViewportChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub data_radix_scroll_area_viewport: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn ScrollAreaViewport(props: &ScrollAreaViewportProps) -> Html {
    let context = use_context::<ScrollAreaContextValue>().expect("Scroll area context required.");
    let viewport_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[
        props.node_ref.clone(),
        viewport_ref.clone(),
        context.viewport_ref,
    ]);

    let child_props = ScrollAreaViewportChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        data_radix_scroll_area_viewport: "".to_owned(),
        id: props.id.clone(),
        style: props.style.clone().with_defaults([
            // We don't support `visible` because the intention is to have at least one scrollbar
            // if this component is used and `visible` will behave like `auto` in that case.
            // https://developer.mozilla.org/en-US/docs/Web/CSS/overflow#description
            //
            // We don't handle `auto` because the intention is for the native implementation
            // to be hidden if using this component. We just want to ensure the node is scrollable
            // so could have used either `scroll` or `auto` here. We picked `scroll` to prevent
            // the browser from having to work out whether to render native scrollbars or not,
            // we tell it to with the intention of hiding them in CSS.
            (
                "overflow-x",
                if context.scrollbar_x_enabled {
                    "scroll"
                } else {
                    "hidden "
                },
            ),
            (
                "overflow-y",
                if context.scrollbar_y_enabled {
                    "scroll"
                } else {
                    "hidden "
                },
            ),
        ]),
    };

    html! {
        <>
            // Hide scrollbars cross-browser and enable momentum scroll for touch devices.
            <style nonce={props.nonce.clone()}>
                {"[data-radix-scroll-area-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}\
                [data-radix-scroll-area-viewport]::-webkit-scrollbar{display:none}"}
            </style>


            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(html! {
                    // `display: table` ensures our content div will match the size of its children in both
                    // horizontal and vertical axis so we can determine if scroll width/height changed and
                    // recalculate thumb sizes. This doesn't account for children with *percentage*
                    // widths that change. We'll wait to see what use-cases consumers come up with there
                    // before trying to resolve it.
                    <div
                        ref={context.content_ref}
                        style={Style::from([
                            ("min-width", "100%"),
                            ("display", "table"),
                        ])}
                    >
                        {props.children.clone()}
                    </div>
                })}
            }
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct ScrollAreaScrollbarProps {
    // TODO: force mount

    // Props from `ScrollAreaScrollbarVisible`
    #[prop_or_default]
    pub orientation: ScrollAreaOrientation,

    // TODO: other props?

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaScrollbarImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ScrollAreaScrollbar(_props: &ScrollAreaScrollbarProps) -> Html {
    html! {}
}

#[derive(PartialEq, Properties)]
struct ScrollAreaScrollbarHoverProps {
    // TODO: force mount

    // Props from `ScrollAreaScrollbarAuto`
    #[prop_or_default]
    pub orientation: ScrollAreaOrientation,

    // Props from `ScrollAreaScrollbarX` / `ScrollAreaScollbarY`
    pub sizes: Sizes,
    pub has_thumb: bool,
    pub on_thumb_change: Callback<Option<web_sys::Element>>,
    pub on_thumb_pointer_up: Callback<()>,
    pub on_thumb_pointer_down: Callback<PointerPos>,
    pub on_thumb_pointer_change: Callback<()>,
    pub on_wheel_scroll: Callback<f64>,
    pub on_drag_scroll: Callback<f64>,
    pub on_resize: Callback<()>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaScrollbarImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn ScrollAreaScrollbarHover(_props: &ScrollAreaScrollbarHoverProps) -> Html {
    html! {}
}

#[derive(PartialEq, Properties)]
struct ScrollAreaScrollbarScrollProps {
    // TODO: force mount

    // Props from `ScrollAreaScrollbarVisible`
    #[prop_or_default]
    pub orientation: ScrollAreaOrientation,

    // Props from `ScrollAreaScrollbarX` / `ScrollAreaScollbarY`
    pub sizes: Sizes,
    pub has_thumb: bool,
    pub on_thumb_change: Callback<Option<web_sys::Element>>,
    pub on_thumb_pointer_up: Callback<()>,
    pub on_thumb_pointer_down: Callback<PointerPos>,
    pub on_thumb_pointer_change: Callback<()>,
    pub on_wheel_scroll: Callback<f64>,
    pub on_drag_scroll: Callback<f64>,
    pub on_resize: Callback<()>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaScrollbarImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn ScrollAreaScrollbarScroll(_props: &ScrollAreaScrollbarScrollProps) -> Html {
    html! {}
}

#[derive(PartialEq, Properties)]
struct ScrollAreaScrollbarAutoProps {
    // TODO: force mount

    // Props from `ScrollAreaScrollbarVisible`
    #[prop_or_default]
    pub orientation: ScrollAreaOrientation,

    // Props from `ScrollAreaScrollbarX` / `ScrollAreaScollbarY`
    pub sizes: Sizes,
    pub has_thumb: bool,
    pub on_thumb_change: Callback<Option<web_sys::Element>>,
    pub on_thumb_pointer_up: Callback<()>,
    pub on_thumb_pointer_down: Callback<PointerPos>,
    pub on_thumb_pointer_change: Callback<()>,
    pub on_wheel_scroll: Callback<f64>,
    pub on_drag_scroll: Callback<f64>,
    pub on_resize: Callback<()>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaScrollbarImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn ScrollAreaScrollbarAuto(_props: &ScrollAreaScrollbarAutoProps) -> Html {
    html! {}
}

#[derive(PartialEq, Properties)]
struct ScrollAreaScrollbarVisibleProps {
    #[prop_or_default]
    pub orientation: ScrollAreaOrientation,

    // Props from `ScrollAreaScrollbarX` / `ScrollAreaScollbarY`
    pub sizes: Sizes,
    pub has_thumb: bool,
    pub on_thumb_change: Callback<Option<web_sys::Element>>,
    pub on_thumb_pointer_up: Callback<()>,
    pub on_thumb_pointer_down: Callback<PointerPos>,
    pub on_thumb_pointer_change: Callback<()>,
    pub on_wheel_scroll: Callback<f64>,
    pub on_drag_scroll: Callback<f64>,
    pub on_resize: Callback<()>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaScrollbarImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn ScrollAreaScrollbarVisible(_props: &ScrollAreaScrollbarVisibleProps) -> Html {
    html! {}
}

#[derive(PartialEq, Properties)]
struct ScrollAreaScrollbarAxisProps {
    pub sizes: Sizes,
    pub has_thumb: bool,
    pub on_thumb_change: Callback<Option<web_sys::Element>>,
    pub on_thumb_pointer_up: Callback<()>,
    pub on_thumb_pointer_down: Callback<PointerPos>,
    pub on_thumb_pointer_change: Callback<()>,
    pub on_wheel_scroll: Callback<f64>,
    pub on_drag_scroll: Callback<f64>,
    pub on_resize: Callback<()>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaScrollbarImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn ScrollAreaScrollbarX(_props: &ScrollAreaScrollbarAxisProps) -> Html {
    html! {}
}

#[function_component]
fn ScrollAreaScrollbarY(_props: &ScrollAreaScrollbarAxisProps) -> Html {
    html! {}
}

#[derive(Clone, Debug, PartialEq)]
pub struct PointerPos {
    x: f64,
    y: f64,
}

#[derive(Clone, PartialEq)]
struct ScrollbarContextValue {
    has_thumb: bool,
    scrollbar_ref: NodeRef,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<PointerPos>,
    on_thumb_pointer_change: Callback<()>,
}

#[derive(PartialEq, Properties)]
struct ScrollAreaScrollbarImplProps {
    pub sizes: Sizes,
    pub has_thumb: bool,
    // TODO: ref?
    pub on_thumb_change: Callback<Option<web_sys::Element>>,
    pub on_thumb_pointer_up: Callback<()>,
    pub on_thumb_pointer_down: Callback<PointerPos>,
    pub on_thumb_pointer_change: Callback<()>,
    pub on_wheel_scroll: Callback<(WheelEvent, f64)>,
    pub on_drag_scroll: Callback<PointerPos>,
    pub on_resize: Callback<()>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaScrollbarImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct ScrollAreaScrollbarImplChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,
    // Event handler attributes
    // TODO
    // pub onpointerdown: Callback<PointerEvent>,
    // pub onpointermove: Callback<PointerEvent>,
    // pub onpointerup: Callback<PointerEvent>,
}

#[function_component]
fn ScrollAreaScrollbarImpl(props: &ScrollAreaScrollbarImplProps) -> Html {
    let _context = use_context::<ScrollAreaContextValue>().expect("Scroll area context required.");
    let scrollbar_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), scrollbar_ref.clone()]);
    let _rect_ref = use_mut_ref(|| None::<web_sys::DomRect>);
    let _prev_webkit_user_select_ref = use_mut_ref(|| "".to_owned());
    // TODO

    let context_value = use_memo(
        (
            props.has_thumb,
            props.on_thumb_pointer_up.clone(),
            props.on_thumb_pointer_down.clone(),
            props.on_thumb_pointer_change.clone(),
            scrollbar_ref,
        ),
        |(
            has_thumb,
            on_thumb_pointer_up,
            on_thumb_pointer_down,
            on_thumb_pointer_change,
            scrollbar_ref,
        )| ScrollbarContextValue {
            has_thumb: *has_thumb,
            scrollbar_ref: scrollbar_ref.clone(),
            on_thumb_pointer_up: on_thumb_pointer_up.clone(),
            on_thumb_pointer_down: on_thumb_pointer_down.clone(),
            on_thumb_pointer_change: on_thumb_pointer_change.clone(),
        },
    );

    let child_props = ScrollAreaScrollbarImplChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes,
        class: props.class.clone(),
        id: props.id.clone(),
        style: props
            .style
            .clone()
            .with_defaults([("position", "absolute")]),
    };

    html! {
        <ContextProvider<ScrollbarContextValue> context={(*context_value).clone()}>
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        </ContextProvider<ScrollbarContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct ScrollAreaThumbProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaThumbChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type ScrollAreaThumbChildProps = ScrollAreaThumbImplChildProps;

#[function_component]
pub fn ScrollAreaThumb(props: &ScrollAreaThumbProps) -> Html {
    html! {
        // TODO: Presence
        <ScrollAreaThumbImpl
            class={props.class.clone()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
        </ScrollAreaThumbImpl>
    }
}

#[derive(PartialEq, Properties)]
struct ScrollAreaThumbImplProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaThumbImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct ScrollAreaThumbImplChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub data_state: String,
    pub id: Option<String>,
    pub style: Style,
    // Event handler attributes
    // TODO
    // pub onpointerdown: Callback<PointerEvent>,
    // pub onpointerup: Callback<PointerEvent>,
}

#[function_component]
fn ScrollAreaThumbImpl(props: &ScrollAreaThumbImplProps) -> Html {
    let _scroll_area_context =
        use_context::<ScrollAreaContextValue>().expect("Scroll area context required.");
    let scrollbar_context =
        use_context::<ScrollbarContextValue>().expect("Scrollbar context required.");

    // TODO

    let child_props = ScrollAreaThumbImplChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes,
        class: props.class.clone(),
        data_state: if scrollbar_context.has_thumb {
            "visible"
        } else {
            "hidden"
        }
        .to_owned(),
        id: props.id.clone(),
        style: props.style.clone().with_defaults([
            ("width", "var(--radix-scroll-area-thumb-width)"),
            ("height", "var(--radix-scroll-area-thumb-height)"),
        ]),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct ScrollAreaCornerProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaCornerChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type ScrollAreaCornerChildProps = ScrollAreaCornerImplChildProps;

#[function_component]
pub fn ScrollAreaCorner(props: &ScrollAreaCornerProps) -> Html {
    let context = use_context::<ScrollAreaContextValue>().expect("Scroll area context required.");
    let has_both_scrollbars_visible =
        context.scrollbar_x_ref.get().is_some() && context.scrollbar_y_ref.get().is_some();
    let has_corner = context.r#type != ScrollAreaType::Scroll && has_both_scrollbars_visible;

    html! {
        if has_corner {
            <ScrollAreaCornerImpl
                class={props.class.clone()}
                id={props.id.clone()}
                style={props.style.clone()}

                node_ref={props.node_ref.clone()}
                attributes={props.attributes.clone()}
                as_child={props.as_child.clone()}
            >
                {props.children.clone()}
            </ScrollAreaCornerImpl>
        }
    }
}

#[derive(PartialEq, Properties)]
struct ScrollAreaCornerImplProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ScrollAreaCornerImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct ScrollAreaCornerImplChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
fn ScrollAreaCornerImpl(props: &ScrollAreaCornerImplProps) -> Html {
    let context = use_context::<ScrollAreaContextValue>().expect("Scroll area context required.");
    let width = use_state_eq(|| 0.0);
    let height = use_state_eq(|| 0.0);
    let has_size = *width != 0.0 && *height != 0.0;

    // TODO: hooks

    let child_props = ScrollAreaCornerImplChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes,
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone().with_defaults([
            ("width", Some(format!("{}px", *width))),
            ("height", Some(format!("{}px", *height))),
            ("position", Some("absolute".to_owned())),
            (
                "right",
                (context.dir == Direction::Ltr).then_some("0px".to_owned()),
            ),
            (
                "left",
                (context.dir == Direction::Rtl).then_some("0px".to_owned()),
            ),
            ("bottom", Some("0px".to_owned())),
        ]),
    };

    html! {
        if has_size {
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        }
    }
}
