use floating_ui_leptos::{
    use_floating, Alignment, ApplyState, Arrow, ArrowData, ArrowOptions, DetectOverflowOptions,
    Flip, FlipOptions, Hide, HideData, HideOptions, HideStrategy, IntoReference, LimitShift,
    LimitShiftOptions, Middleware, MiddlewareReturn, MiddlewareState, MiddlewareVec, Offset,
    OffsetOptions, OffsetOptionsValues, Padding, Placement, Shift, ShiftOptions, Side, Size,
    SizeOptions, Strategy, UseFloatingOptions, UseFloatingReturn, ARROW_NAME, HIDE_NAME,
};
use leptos::{
    html::{AnyElement, Div, Span},
    *,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sticky {
    Partial,
    Always,
}

#[component]
pub fn Popper(children: Children) -> impl IntoView {
    // TODO: provide context

    view! {
        {children()}
    }
}

#[component]
pub fn PopperAnchor(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || class.get() {..attributes}>
            {children()}
        </div>
    }
}

#[component]
pub fn PopperContent(
    #[prop(into, optional)] side: MaybeProp<Side>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] align: MaybeProp<Align>,
    #[prop(into, optional)] align_offset: MaybeProp<f64>,
    #[prop(into, optional)] arrow_padding: MaybeProp<f64>,
    #[prop(into, optional)] avoid_collisions: MaybeProp<bool>,
    // collision_boundary
    #[prop(into, optional)] collision_padding: MaybeProp<Padding>,
    #[prop(into, optional)] sticky: MaybeProp<Sticky>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let side = move || side().unwrap_or(Side::Bottom);
    let side_offset = move || side_offset().unwrap_or(0.0);
    let align = move || align().unwrap_or(Align::Center);
    let align_offset = move || align_offset().unwrap_or(0.0);
    let arrow_padding = move || arrow_padding().unwrap_or(0.0);
    let avoid_collisions = move || avoid_collisions().unwrap_or(true);
    // collision_boundary
    let collision_padding = move || collision_padding().unwrap_or(Padding::All(0.0));
    let sticky = move || sticky().unwrap_or(Sticky::Partial);
    let hide_when_detached = move || hide_when_detached().unwrap_or(false);

    let desired_placement = Signal::derive(move || Placement::from((side(), align().alignment())));

    let arrow = create_node_ref::<Span>();
    let arrow_size = 0.0;
    let arrow_width = 0.0;
    let arrow_height = 0.0;

    let boundary = move || vec![0];
    let has_explicit_boundaries = move || boundary().len() > 0;

    let reference_ref = create_node_ref::<AnyElement>();
    let floating_ref = create_node_ref::<Div>();

    let UseFloatingReturn {
        floating_styles,
        placement,
        is_positioned,
        middleware_data,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed.into())
            .placement(desired_placement.into())
            .middleware(MaybeProp::derive(move || {
                let detect_overflow_options = DetectOverflowOptions::default()
                    .padding(collision_padding())
                    // TODO: .boundary(value)
                    .alt_boundary(has_explicit_boundaries());

                let mut middleware: MiddlewareVec =
                    vec![Box::new(Offset::new(OffsetOptions::Values(
                        OffsetOptionsValues::default()
                            .main_axis(side_offset() + arrow_height)
                            .alignment_axis(align_offset()),
                    )))];

                if avoid_collisions() {
                    let mut shift_options = ShiftOptions::default()
                        .detect_overflow(detect_overflow_options.clone())
                        .main_axis(true)
                        .cross_axis(false);

                    if sticky() == Sticky::Partial {
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
                                    &format!("{}px", available_width),
                                )
                                .expect("Style should be updated.");
                            content_style
                                .set_property(
                                    "--radix-popper-available-height",
                                    &format!("{}px", available_height),
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
                    ArrowOptions::new(arrow).padding(Padding::All(arrow_padding())),
                )));

                middleware.push(Box::new(TransformOrigin::new(TransformOriginOptions {
                    arrow_width,
                    arrow_height,
                })));

                if hide_when_detached() {
                    middleware.push(Box::new(Hide::new(
                        HideOptions::default()
                            .detect_overflow(detect_overflow_options)
                            .strategy(HideStrategy::ReferenceHidden),
                    )));
                }

                Some(middleware)
            })),
    );

    let placed_side = move || placement().side();
    let placed_align = move || Align::from(placement().alignment());

    // TODO: handlePlaced

    let arrow_data = move || -> Option<ArrowData> { middleware_data().get_as(ARROW_NAME) };
    let arrow_x = move || arrow_data().and_then(|arrow_data| arrow_data.x);
    let arrow_y = move || arrow_data().and_then(|arrow_data| arrow_data.y);
    let cannot_center_arrow =
        move || arrow_data().map_or(true, |arrow_data| arrow_data.center_offset != 0.0);

    // TODO
    let content_z_index = move || "0";

    // TODO: move this to FloatingStyles
    let style_attributes = vec![
        (
            "style:position",
            (move || {
                (match floating_styles().position {
                    Strategy::Absolute => "absolute",
                    Strategy::Fixed => "fixed",
                })
                .to_string()
                .into_attribute()
            })
            .into_attribute(),
        ),
        (
            "style:top",
            (move || floating_styles().top).into_attribute(),
        ),
        (
            "style:left",
            (move || floating_styles().left).into_attribute(),
        ),
        (
            "style:transform",
            (move || floating_styles().transform).into_attribute(),
        ),
        (
            "style:will-change",
            (move || floating_styles().will_change).into_attribute(),
        ),
    ];

    let transform_origin_data =
        move || -> Option<TransformOriginData> { middleware_data().get_as(TRANSFORM_ORIGIN_NAME) };
    let hide_data = move || -> Option<HideData> { middleware_data().get_as(HIDE_NAME) };
    let reference_hidden = move || {
        hide_data()
            .and_then(|hide_data| hide_data.reference_hidden)
            .unwrap_or(false)
    };

    view! {
        // TODO: add Floating UI
        <div
            _ref={floating_ref}
            {..style_attributes}
            style:transform=move || match is_positioned() {
                true => floating_styles().transform,
                // Keep off the page when measuring
                false => Some("translate(0, -200%)".into())
            }
            style:min-width="max-content"
            style:z-index=content_z_index
            // TODO: --radix-popper-transform-origin

            // Hide the content if using the hide middleware and should be hidden set visibility to hidden
            // and disable pointer events so the UI behaves as if the PopperContent isn't there at all.
            style:visibility=move || match reference_hidden() {
                true => Some("hidden"),
                false => None,
            }
            style:pointer-events=move || match reference_hidden() {
                true => Some("none"),
                false => None,
            }

            // Floating UI interally calculates logical alignment based the `dir` attribute on
            // the reference/floating node, we must add this attribute here to ensure
            // this is calculated when portalled as well as inline.
            // TODO
            // dir={dir}
        >
            <div class=move || class.get() {..attributes}>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn PopperArrow(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {}
}

const TRANSFORM_ORIGIN_NAME: &str = "transformOrigin";

/// Options for [`TransformOrigin`] middleware.
#[derive(Clone)]
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

#[derive(Clone)]
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
            .map_or(true, |arrow_data| arrow_data.center_offset != 0.0);
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
                    false => format!("{}px", arrow_x_center),
                },
                format!("{}px", rects.floating.height + arrow_height),
            ),
            Side::Right => (
                format!("{}px", -arrow_height),
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{}px", arrow_y_center),
                },
            ),
            Side::Bottom => (
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{}px", arrow_x_center),
                },
                format!("{}px", -arrow_height),
            ),
            Side::Left => (
                format!("{}px", rects.floating.width + arrow_height),
                match is_arrow_hidden {
                    true => no_arrow_align.into(),
                    false => format!("{}px", arrow_y_center),
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
