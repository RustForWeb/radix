use floating_ui_leptos::{
    ARROW_NAME, Alignment, ApplyState, Arrow, ArrowData, ArrowOptions, AutoUpdateOptions, Boundary,
    DetectOverflowOptions, Flip, FlipOptions, HIDE_NAME, Hide, HideData, HideOptions, HideStrategy,
    LimitShift, LimitShiftOptions, Middleware, MiddlewareReturn, MiddlewareState, MiddlewareVec,
    Offset, OffsetOptions, OffsetOptionsValues, Padding, Placement, Shift, ShiftOptions, Side,
    Size, SizeOptions, Strategy, UseFloatingOptions, UseFloatingReturn, use_floating,
};
use leptos::{attribute_interceptor::AttributeInterceptor, context::Provider, html, prelude::*};
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_arrow::Arrow as ArrowPrimitive;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_primitive::Primitive;
use radix_leptos_use_size::use_size;
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Align {
    pub fn alignment(self) -> Option<Alignment> {
        match self {
            Align::Start => Some(Alignment::Start),
            Align::Center => None,
            Align::End => Some(Alignment::End),
        }
    }
}

impl From<Option<Alignment>> for Align {
    fn from(value: Option<Alignment>) -> Self {
        match value {
            Some(Alignment::Start) => Align::Start,
            Some(Alignment::End) => Align::End,
            None => Align::Center,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Sticky {
    Partial,
    Always,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UpdatePositionStrategy {
    Optimized,
    Always,
}

#[derive(Clone)]
struct PopperContextValue {
    pub anchor_ref: AnyNodeRef,
}

#[component]
pub fn Popper(children: ChildrenFn) -> impl IntoView {
    let anchor_ref = AnyNodeRef::new();

    let context_value = PopperContextValue { anchor_ref };

    view! {
        <Provider value={context_value}>
            {children()}
        </Provider>
    }
}

#[component]
pub fn PopperAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context: PopperContextValue = expect_context();
    let composed_refs = use_composed_refs(vec![node_ref, context.anchor_ref]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=composed_refs
        >
            {children()}
        </Primitive>
    }
}

#[derive(Clone)]
struct PopperContentContextValue {
    pub placed_side: Signal<Side>,
    pub arrow_ref: AnyNodeRef,
    pub arrow_x: Signal<Option<f64>>,
    pub arrow_y: Signal<Option<f64>>,
    pub should_hide_arrow: Signal<bool>,
}

#[component]
pub fn PopperContent(
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Center.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(0.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] on_placed: MaybeCallback<()>,
    #[prop(into, optional)] dir: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context: PopperContextValue = expect_context();

    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    let arrow_ref = AnyNodeRef::new();
    let arrow_size = use_size(arrow_ref);
    let arrow_width = move || {
        arrow_size
            .get()
            .map(|arrow_size| arrow_size.width)
            .unwrap_or(0.0)
    };
    let arrow_height = move || {
        arrow_size
            .get()
            .map(|arrow_size| arrow_size.height)
            .unwrap_or(0.0)
    };

    let desired_placement =
        Signal::derive(move || Placement::from((side.get(), align.get().alignment())));

    let floating_ref = AnyNodeRef::new();

    let UseFloatingReturn {
        floating_styles,
        placement,
        is_positioned,
        middleware_data,
        ..
    } = use_floating(
        context.anchor_ref,
        floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed)
            .placement(desired_placement)
            .while_elements_mounted_auto_update_with_options(Signal::derive(move || {
                AutoUpdateOptions::default().animation_frame(
                    update_position_strategy.get() == UpdatePositionStrategy::Always,
                )
            }))
            .middleware(MaybeProp::derive(move || {
                let detect_overflow_options = DetectOverflowOptions::default()
                    .padding(collision_padding.get())
                    .boundary(Boundary::Elements((*collision_boundary.get()).clone()))
                    .alt_boundary(!collision_boundary.get().is_empty());

                let mut middleware: MiddlewareVec =
                    vec![Box::new(Offset::new(OffsetOptions::Values(
                        OffsetOptionsValues::default()
                            .main_axis(side_offset.get() + arrow_height())
                            .alignment_axis(align_offset.get()),
                    )))];

                if avoid_collisions.get() {
                    let mut shift_options = ShiftOptions::default()
                        .detect_overflow(detect_overflow_options.clone())
                        .main_axis(true)
                        .cross_axis(false);

                    if sticky.get() == Sticky::Partial {
                        shift_options = shift_options
                            .limiter(Box::new(LimitShift::new(LimitShiftOptions::default())));
                    }

                    middleware.push(Box::new(Shift::new(shift_options)));

                    middleware.push(Box::new(Flip::new(
                        FlipOptions::default().detect_overflow(detect_overflow_options.clone()),
                    )));
                }

                middleware.push(Box::new(Size::new(
                    SizeOptions::default()
                        .detect_overflow(detect_overflow_options.clone())
                        .apply(&|ApplyState {
                                     state,
                                     available_width,
                                     available_height,
                                 }| {
                            let MiddlewareState {
                                elements, rects, ..
                            } = state;

                            let content_style = (*elements.floating)
                                .clone()
                                .unchecked_into::<web_sys::HtmlElement>()
                                .style();

                            content_style
                                .set_property(
                                    "--radix-popper-available-width",
                                    &format!("{available_width}px"),
                                )
                                .expect("Style should be updated.");
                            content_style
                                .set_property(
                                    "--radix-popper-available-height",
                                    &format!("{available_height}px"),
                                )
                                .expect("Style should be updated.");
                            content_style
                                .set_property(
                                    "--radix-popper-anchor-width",
                                    &format!("{}px", rects.reference.width),
                                )
                                .expect("Style should be updated.");
                            content_style
                                .set_property(
                                    "--radix-popper-anchor-height",
                                    &format!("{}px", rects.reference.height),
                                )
                                .expect("Style should be updated.");
                        }),
                )));

                middleware.push(Box::new(Arrow::new(
                    ArrowOptions::new(arrow_ref).padding(Padding::All(arrow_padding.get())),
                )));

                middleware.push(Box::new(TransformOrigin::new(TransformOriginOptions {
                    arrow_width: arrow_width(),
                    arrow_height: arrow_height(),
                })));

                if hide_when_detached.get() {
                    middleware.push(Box::new(Hide::new(
                        HideOptions::default()
                            .detect_overflow(detect_overflow_options)
                            .strategy(HideStrategy::ReferenceHidden),
                    )));
                }

                Some(SendWrapper::new(middleware))
            })),
    );

    let placed_side = Signal::derive(move || placement.get().side());
    let placed_align = move || Align::from(placement.get().alignment());

    Effect::new(move |_| {
        if is_positioned.get() {
            on_placed.run(());
        }
    });

    let arrow_data = move || -> Option<ArrowData> { middleware_data.get().get_as(ARROW_NAME) };
    let arrow_x = Signal::derive(move || arrow_data().and_then(|arrow_data| arrow_data.x));
    let arrow_y = Signal::derive(move || arrow_data().and_then(|arrow_data| arrow_data.y));
    let cannot_center_arrow = Signal::derive(move || {
        arrow_data().is_none_or(|arrow_data| arrow_data.center_offset != 0.0)
    });

    let (content_z_index, set_content_z_index) = signal::<Option<String>>(None);
    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            set_content_z_index.set(Some(
                window()
                    .get_computed_style(&content)
                    .expect("Element is valid.")
                    .expect("Element should have computed style.")
                    .get_property_value("z-index")
                    .expect("Computed style should have z-index."),
            ));
        }
    });

    let transform_origin_data = move || -> Option<TransformOriginData> {
        middleware_data.get().get_as(TRANSFORM_ORIGIN_NAME)
    };
    let transform_origin = move || {
        transform_origin_data().map(|transform_origin_data| {
            format!("{} {}", transform_origin_data.x, transform_origin_data.y)
        })
    };
    let hide_data = move || -> Option<HideData> { middleware_data.get().get_as(HIDE_NAME) };
    let reference_hidden = move || {
        hide_data()
            .and_then(|hide_data| hide_data.reference_hidden)
            .unwrap_or(false)
    };

    let content_context_value = PopperContentContextValue {
        placed_side,
        arrow_ref,
        arrow_x,
        arrow_y,
        should_hide_arrow: cannot_center_arrow,
    };

    view! {
        <AttributeInterceptor let:attrs>
            <div
                node_ref={floating_ref}
                data-radix-popper-content-wrapper=""
                style:position=move || floating_styles.get().style_position()
                style:top=move || floating_styles.get().style_top()
                style:left=move || floating_styles.get().style_left()
                style:transform=move || match is_positioned.get() {
                    true => floating_styles.get().style_transform(),
                    // Keep off the page when measuring
                    false => Some("translate(0, -200%)".into())
                }
                style:will-change=move || floating_styles.get().style_will_change()
                style:min-width="max-content"
                style:z-index=content_z_index
                style=("--radix-popper-transform-origin", transform_origin)

                // Hide the content if using the hide middleware and should be hidden set visibility to hidden
                // and disable pointer events so the UI behaves as if the PopperContent isn't there at all.
                style:visibility=move || reference_hidden().then_some("hidden")
                style:pointer-events=move || reference_hidden().then_some("none")

                // Floating UI interally calculates logical alignment based the `dir` attribute on
                // the reference/floating node, we must add this attribute here to ensure
                // this is calculated when portalled as well as inline.
                dir=move || dir.get()
            >
                <Provider value={content_context_value.clone()}>
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref={composed_refs}
                        {..attrs}
                        attr:data-side=move || format!("{:?}", placed_side.get()).to_lowercase()
                        attr:data-align=move || format!("{:?}", placed_align()).to_lowercase()
                        // If the PopperContent hasn't been placed yet (not all measurements done),
                        // we prevent animations so that users's animation don't kick in too early referring wrong sides.
                        style:animation=move || (!is_positioned.get()).then_some("none")
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                </Provider>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
pub fn PopperArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let content_context: PopperContentContextValue = expect_context();
    let base_side = move || content_context.placed_side.get().opposite();

    view! {
        <AttributeInterceptor let:attrs>
            // We have to use an extra wrapper, because `ResizeObserver` (used by `useSize`)
            // doesn't report size as we'd expect on SVG elements.
            // It reports their bounding box, which is effectively the largest path inside the SVG.
            <span
                node_ref=content_context.arrow_ref
                style:position="absolute"
                style:left=move || match base_side() {
                    Side::Left => Some("0px".into()),
                    _ => content_context.arrow_x.get().map(|arrow_x| format!("{arrow_x}px"))
                }
                style:top=move || match base_side() {
                    Side::Top => Some("0px".into()),
                    _ => content_context.arrow_y.get().map(|arrow_y| format!("{arrow_y}px"))
                }
                style:right=move || match base_side() {
                    Side::Right => Some("0px"),
                    _ => None
                }
                style:bottom=move || match base_side() {
                    Side::Bottom => Some("0px"),
                    _ => None
                }
                style:transform-origin=move || match content_context.placed_side.get() {
                    Side::Top => "",
                    Side::Right => "0 0",
                    Side::Bottom => "center 0",
                    Side::Left => "100% 0",
                }
                style:transform=move || match content_context.placed_side.get() {
                    Side::Top => "translateY(100%)",
                    Side::Right => "translateY(50%) rotate(90deg) translateX(-50%)",
                    Side::Bottom => "rotate(180deg)",
                    Side::Left => "translateY(50%) rotate(-90deg) translateX(50%)",
                }
                style:visibility=move || content_context.should_hide_arrow.get().then_some("hidden")
            >
                <ArrowPrimitive
                    width=width
                    height=height
                    as_child=as_child
                    node_ref={node_ref}
                    {..attrs}
                    // Ensures the element can be measured correctly (mostly for if SVG).
                    style:display="block"
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </ArrowPrimitive>
            </span>
        </AttributeInterceptor>
    }
}

const TRANSFORM_ORIGIN_NAME: &str = "transformOrigin";

/// Options for [`TransformOrigin`] middleware.
#[derive(Clone, PartialEq)]
struct TransformOriginOptions {
    arrow_width: f64,
    arrow_height: f64,
}

/// Data stored by [`TransformOrigin`] middleware.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TransformOriginData {
    pub x: String,
    pub y: String,
}

#[derive(Clone, PartialEq)]
struct TransformOrigin {
    options: TransformOriginOptions,
}

impl TransformOrigin {
    fn new(options: TransformOriginOptions) -> Self {
        Self { options }
    }
}

impl Middleware<web_sys::Element, web_sys::Window> for TransformOrigin {
    fn name(&self) -> &'static str {
        TRANSFORM_ORIGIN_NAME
    }

    fn compute(
        &self,
        state: MiddlewareState<web_sys::Element, web_sys::Window>,
    ) -> MiddlewareReturn {
        let MiddlewareState {
            placement,
            rects,
            middleware_data,
            ..
        } = state;

        let arrow_data: Option<ArrowData> = middleware_data.get_as(ARROW_NAME);
        let cannot_center_arrow = arrow_data
            .as_ref()
            .is_none_or(|arrow_data| arrow_data.center_offset != 0.0);
        let is_arrow_hidden = cannot_center_arrow;
        let arrow_width = match is_arrow_hidden {
            true => 0.0,
            false => self.options.arrow_width,
        };
        let arrow_height = match is_arrow_hidden {
            true => 0.0,
            false => self.options.arrow_height,
        };

        let placed_side = placement.side();
        let placed_align = Align::from(placement.alignment());
        let no_arrow_align = match placed_align {
            Align::Start => "0%",
            Align::Center => "50%",
            Align::End => "100%",
        };

        let arrow_x_center = arrow_data
            .as_ref()
            .and_then(|arrow_data| arrow_data.x)
            .unwrap_or(0.0)
            + arrow_width / 2.0;
        let arrow_y_center = arrow_data
            .as_ref()
            .and_then(|arrow_data| arrow_data.y)
            .unwrap_or(0.0)
            + arrow_height / 2.0;

        let (x, y) = match placed_side {
            Side::Top => (
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{arrow_x_center}px"),
                },
                format!("{}px", rects.floating.height + arrow_height),
            ),
            Side::Right => (
                format!("{}px", -arrow_height),
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{arrow_y_center}px"),
                },
            ),
            Side::Bottom => (
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{arrow_x_center}px"),
                },
                format!("{}px", -arrow_height),
            ),
            Side::Left => (
                format!("{}px", rects.floating.width + arrow_height),
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{arrow_y_center}px"),
                },
            ),
        };

        MiddlewareReturn {
            x: None,
            y: None,
            data: Some(
                serde_json::to_value(TransformOriginData { x, y })
                    .expect("Data should be valid JSON."),
            ),
            reset: None,
        }
    }
}
