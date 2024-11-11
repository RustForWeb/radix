use floating_ui_yew::{
    use_auto_update_with_options, use_floating, Alignment, ApplyState, Arrow, ArrowData,
    ArrowOptions, AutoUpdateOptions, Boundary, DetectOverflowOptions, Flip, FlipOptions, Hide,
    HideData, HideOptions, HideStrategy, LimitShift, LimitShiftOptions, Middleware,
    MiddlewareReturn, MiddlewareState, MiddlewareVec, Offset, OffsetOptions, OffsetOptionsValues,
    Padding, Placement, Shift, ShiftOptions, Side, Size, SizeOptions, Strategy, UseFloatingOptions,
    UseFloatingReturn, ARROW_NAME, HIDE_NAME,
};
use radix_yew_arrow::{Arrow as ArrowPrimitive, ArrowChildProps, SetArrowChildProps};
use radix_yew_use_size::use_size;
use serde::{Deserialize, Serialize};
use web_sys::{wasm_bindgen::JsCast, window};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};

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

#[derive(Clone, PartialEq)]
struct PopperContextValue {
    pub anchor_ref: NodeRef,
}

#[derive(PartialEq, Properties)]
pub struct PopperProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Popper(props: &PopperProps) -> Html {
    let anchor_ref = use_node_ref();

    let context_value = use_memo(anchor_ref, |anchor_ref| PopperContextValue {
        anchor_ref: (*anchor_ref).clone(),
    });

    html! {
        <ContextProvider<PopperContextValue> context={(*context_value).clone()}>
            {props.children.clone()}
        </ContextProvider<PopperContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct PopperAnchorProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<PopperAnchorChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct PopperAnchorChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,

    // Event handler attributes
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn PopperAnchor(props: &PopperAnchorProps) -> Html {
    let context = use_context::<PopperContextValue>().expect("Popper context required.");
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), context.anchor_ref]);

    let child_props = PopperAnchorChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone(),

        // Event handler attributes
        onclick: props.on_click.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(Clone, PartialEq)]
struct PopperContentContextValue {
    pub placed_side: Side,
    pub arrow_ref: NodeRef,
    pub arrow_x: Option<f64>,
    pub arrow_y: Option<f64>,
    pub should_hide_arrow: bool,
}

#[derive(PartialEq, Properties)]
pub struct PopperContentProps<ChildProps: Clone + Default + PartialEq + SetPopperContentChildProps>
{
    #[prop_or(Side::Bottom)]
    pub side: Side,
    #[prop_or(0.0)]
    pub side_offset: f64,
    #[prop_or(Align::Center)]
    pub align: Align,
    #[prop_or(0.0)]
    pub align_offset: f64,
    #[prop_or(0.0)]
    pub arrow_padding: f64,
    #[prop_or(true)]
    pub avoid_collisions: bool,
    #[prop_or_default]
    pub collision_boundary: Vec<web_sys::Element>,
    #[prop_or(Padding::All(0.0))]
    pub collision_padding: Padding,
    #[prop_or(Sticky::Partial)]
    pub sticky: Sticky,
    #[prop_or(false)]
    pub hide_when_detached: bool,
    #[prop_or(UpdatePositionStrategy::Optimized)]
    pub update_position_strategy: UpdatePositionStrategy,
    #[prop_or_default]
    pub on_placed: Callback<()>,

    // Global attributes,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub dir: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_context_menu: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,

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

pub trait SetPopperContentChildProps {
    fn set_popper_content_child_props(&mut self, props: PopperContentChildProps);
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct PopperContentChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub data_align: String,
    pub data_side: String,
    pub id: Option<String>,
    pub role: Option<String>,
    pub style: String,

    // Event handler attributes
    pub oncontextmenu: Callback<MouseEvent>,
    pub onkeydown: Callback<KeyboardEvent>,
}

impl SetPopperContentChildProps for PopperContentChildProps {
    fn set_popper_content_child_props(&mut self, _: PopperContentChildProps) {}
}

#[function_component]
pub fn PopperContent<ChildProps = PopperContentChildProps>(
    props: &PopperContentProps<ChildProps>,
) -> Html
where
    ChildProps: Clone + Default + PartialEq + SetPopperContentChildProps,
{
    let context = use_context::<PopperContextValue>().expect("Popper context is required.");

    let content_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);

    let arrow_ref = use_node_ref();
    let arrow_size = use_size(arrow_ref.clone());
    let arrow_width = arrow_size
        .as_ref()
        .map(|arrow_size| arrow_size.width)
        .unwrap_or(0.0);
    let arrow_height = arrow_size
        .as_ref()
        .map(|arrow_size| arrow_size.height)
        .unwrap_or(0.0);

    let desired_placement = Placement::from((props.side, props.align.alignment()));

    let floating_ref = use_node_ref();

    let auto_update = use_auto_update_with_options(
        AutoUpdateOptions::default()
            .animation_frame(props.update_position_strategy == UpdatePositionStrategy::Always),
    );
    let middleware = use_memo(
        (
            props.side_offset,
            props.align_offset,
            props.avoid_collisions,
            props.arrow_padding,
            props.collision_boundary.clone(),
            props.collision_padding.clone(),
            props.sticky,
            props.hide_when_detached,
            arrow_ref.clone(),
            arrow_width,
            arrow_height,
        ),
        |(
            side_offset,
            align_offset,
            avoid_collisions,
            arrow_padding,
            collision_boundary,
            collision_padding,
            sticky,
            hide_when_detached,
            arrow_ref,
            arrow_width,
            arrow_height,
        )| {
            let detect_overflow_options = DetectOverflowOptions::default()
                .padding((*collision_padding).clone())
                .boundary(Boundary::Elements((*collision_boundary).clone()))
                .alt_boundary(!collision_boundary.is_empty());

            let mut middleware: MiddlewareVec = vec![Box::new(Offset::new(OffsetOptions::Values(
                OffsetOptionsValues::default()
                    .main_axis(side_offset + arrow_height)
                    .alignment_axis(*align_offset),
            )))];

            if *avoid_collisions {
                let mut shift_options = ShiftOptions::default()
                    .detect_overflow(detect_overflow_options.clone())
                    .main_axis(true)
                    .cross_axis(false);

                if *sticky == Sticky::Partial {
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
                ArrowOptions::new((*arrow_ref).clone()).padding(Padding::All(*arrow_padding)),
            )));

            middleware.push(Box::new(TransformOrigin::new(TransformOriginOptions {
                arrow_width: *arrow_width,
                arrow_height: *arrow_height,
            })));

            if *hide_when_detached {
                middleware.push(Box::new(Hide::new(
                    HideOptions::default()
                        .detect_overflow(detect_overflow_options)
                        .strategy(HideStrategy::ReferenceHidden),
                )));
            }

            middleware
        },
    );

    let UseFloatingReturn {
        floating_styles,
        placement,
        is_positioned,
        middleware_data,
        ..
    } = use_floating(
        context.anchor_ref.into(),
        floating_ref.clone(),
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed)
            .placement(desired_placement)
            .while_elements_mounted((*auto_update).clone())
            .middleware((*middleware).clone()),
    );

    let placed_side = placement.side();
    let placed_align = Align::from(placement.alignment());

    use_effect_with(is_positioned.clone(), {
        let on_placed = props.on_placed.clone();

        move |is_positioned| {
            if **is_positioned {
                on_placed.emit(());
            }
        }
    });

    let arrow_data = middleware_data.get_as::<ArrowData>(ARROW_NAME);
    let arrow_x = arrow_data.as_ref().and_then(|arrow_data| arrow_data.x);
    let arrow_y = arrow_data.as_ref().and_then(|arrow_data| arrow_data.y);
    let cannot_center_arrow = arrow_data
        .as_ref()
        .map_or(true, |arrow_data| arrow_data.center_offset != 0.0);

    let content_z_index: UseStateHandle<Option<String>> = use_state_eq(|| None);
    use_effect_with(content_ref.clone(), {
        let content_z_index = content_z_index.clone();

        move |content_ref| {
            if let Some(content) = content_ref.cast::<web_sys::Element>() {
                content_z_index.set(Some(
                    window()
                        .expect("Window should exist.")
                        .get_computed_style(&content)
                        .expect("Element is valid.")
                        .expect("Element should have computed style.")
                        .get_property_value("z-index")
                        .expect("Computed style should have z-index."),
                ));
            }
        }
    });

    let transform_origin_data =
        middleware_data.get_as::<TransformOriginData>(TRANSFORM_ORIGIN_NAME);
    let transform_origin = transform_origin_data.map(|transform_origin_data| {
        format!("{} {}", transform_origin_data.x, transform_origin_data.y)
    });
    let hide_data = middleware_data.get_as::<HideData>(HIDE_NAME);
    let reference_hidden = hide_data
        .and_then(|hide_data| hide_data.reference_hidden)
        .unwrap_or(false);

    let content_context_value = PopperContentContextValue {
        placed_side,
        arrow_ref,
        arrow_x,
        arrow_y,
        should_hide_arrow: cannot_center_arrow,
    };

    let child_props = PopperContentChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        data_align: format!("{:?}", placed_align).to_lowercase(),
        data_side: format!("{:?}", placed_side).to_lowercase(),
        id: props.id.clone(),
        role: props.role.clone(),
        // If the PopperContent hasn't been placed yet (not all measurements done),
        // we prevent animations so that users's animation don't kick in too early referring wrong sides.
        style: format!(
            "{}{}",
            (!(*is_positioned))
                .then_some("animation: none;")
                .unwrap_or_default(),
            props.style.clone().unwrap_or_default()
        ),

        // Event handler attributes
        oncontextmenu: props.on_context_menu.clone(),
        onkeydown: props.on_key_down.clone(),
    };

    html! {
        <div
            ref={floating_ref}
            data-radix-popper-content-wrapper=""
            style={format!(
                "{}; min-width: max-content;{}{}{}",
                {
                    let mut floating_styles = (*floating_styles).clone();
                    if *is_positioned {
                        // Keep off the page when measuring
                        floating_styles.transform = Some("translate(0, -200%)".to_owned());
                    }

                    floating_styles
                },
                match content_z_index.as_ref() {
                    Some(content_z_index) => format!(" z-index: {content_z_index};"),
                    None => "".to_owned(),
                },
                match transform_origin.as_ref() {
                    Some(transform_origin) => format!(" --radix-popper-transform-origin: {transform_origin};"),
                    None => "".to_owned()
                },
                // Hide the content if using the hide middleware and should be hidden set visibility to hidden
                // and disable pointer events so the UI behaves as if the PopperContent isn't there at all.
                match reference_hidden {
                    true => " visibility: hidden; pointer-events: none;",
                    false => ""
                }
            )}

            // Floating UI interally calculates logical alignment based the `dir` attribute on
            // the reference/floating node, we must add this attribute here to ensure
            // this is calculated when portalled as well as inline.
            dir={props.dir.clone()}
        >
            <ContextProvider<PopperContentContextValue> context={content_context_value}>
                if let Some(as_child) = props.as_child.as_ref() {
                    {{
                        let mut as_child_props = props.as_child_props.clone().unwrap_or_default();
                        as_child_props.set_popper_content_child_props(child_props);

                        as_child.emit(as_child_props)
                    }}
                } else {
                    {child_props.render(props.children.clone())}
                }
            </ContextProvider<PopperContentContextValue>>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct PopperArrowProps {
    // Props from `Arrow`
    #[prop_or(10.0)]
    pub width: f64,
    #[prop_or(5.0)]
    pub height: f64,

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
    pub as_child: Option<Callback<PopperArrowChildProps, Html>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct PopperArrowChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: String,

    // Attributes from `svg`
    pub width: String,
    pub height: String,
}

impl SetArrowChildProps for PopperArrowChildProps {
    fn set_arrow_child_props(&mut self, props: ArrowChildProps) {
        self.node_ref = props.node_ref;
        self.attributes = props.attributes;

        self.class = props.class;
        self.id = props.id;
        if let Some(style) = props.style {
            self.style = style;
        }

        self.width = props.width;
        self.height = props.height;
    }
}

#[function_component]
pub fn PopperArrow(props: &PopperArrowProps) -> Html {
    let content_context =
        use_context::<PopperContentContextValue>().expect("Popper content context is required.");
    let base_side = content_context.placed_side.opposite();

    let child_props = PopperArrowChildProps {
        style: format!("display: block;{}", props.style.clone().unwrap_or_default()),
        ..PopperArrowChildProps::default()
    };

    html! {
        <span
            ref={content_context.arrow_ref}
            style={format!(
                "position: absolute;{}{}{}{} transform-origin: {}; transform: {};{}",
                match base_side {
                    Side::Left => " left: 0px;".to_owned(),
                    _ => match content_context.arrow_x {
                        Some(arrow_x) => format!(" left: {}px;", arrow_x),
                        None => "".to_owned(),
                    }
                },
                match base_side {
                    Side::Top => " top: 0px;".to_owned(),
                    _ => match content_context.arrow_y {
                        Some(arrow_y) => format!(" top: {}px;", arrow_y),
                        None => "".to_owned(),
                    },
                },
                match base_side {
                    Side::Bottom => " right: 0px;",
                    _ => "",
                },
                match base_side {
                    Side::Bottom => " bottom: 0px;",
                    _ => "",
                },
                match content_context.placed_side {
                    Side::Top => "",
                    Side::Right => "0 0",
                    Side::Bottom => "center 0",
                    Side::Left => "100% 0",
                },
                match content_context.placed_side {
                    Side::Top => "translateY(100%)",
                    Side::Right => "translateY(50%) rotate(90deg) translateX(-50%)",
                    Side::Bottom => "rotate(180deg)",
                    Side::Left => "translateY(50%) rotate(-90deg) translateX(50%)",
                },
                match content_context.should_hide_arrow {
                    true => "visibility: hidden;",
                    false => "",
                }
            )}
        >
        <ArrowPrimitive<PopperArrowChildProps>
            width={props.width}
            height={props.height}

            class={props.class.clone()}
            id={props.id.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={props.as_child.clone()}
            as_child_props={child_props}
        />
    </span>
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
