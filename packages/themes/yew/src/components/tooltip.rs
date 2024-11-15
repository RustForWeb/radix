pub use radix_yew_tooltip::{
    Align as TooltipAlign, Padding as TooltipPadding, Side as TooltipSide, Sticky as TooltipSticky,
    TooltipTriggerChildProps, TooltipTriggerChildProps as TooltipTriggerPrimitiveChildProps,
    UpdatePositionStrategy as TooltipUpdatePositionStrategy,
};
use radix_yew_tooltip::{
    Tooltip as TooltipPrimitive, TooltipArrow as TooltipArrowPrimitive,
    TooltipContent as TooltipContentPrimitive, TooltipPortal as TooltipPortalPrimitive,
    TooltipTrigger as TooltipTriggerPrimitive,
};
use yew::prelude::*;
use yew_struct_component::Attributes;

use crate::{
    components::{
        text::Text,
        text_props::TextAs,
        theme::{Theme, ThemeChildProps},
    },
    helpers::{extract_props::extract_props, merge_styles::Style},
    props::{
        prop_def::Responsive,
        width_props::{MaxWidthProp, MinWidthProp, WidthProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct TooltipProps {
    pub content: Html,
    #[prop_or_default]
    pub width: WidthProp,
    #[prop_or_default]
    pub min_width: MinWidthProp,
    #[prop_or(MaxWidthProp(Some(Responsive::Value("360px".to_owned()))))]
    pub max_width: MaxWidthProp,

    // Props from `TooltipPrimitive`
    #[prop_or_default]
    pub open: Option<bool>,
    #[prop_or_default]
    pub default_open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    /// The duration from when the pointer enters the trigger until the tooltip gets opened. Defaults to `700`.
    #[prop_or_default]
    pub delay_duration: Option<i32>,
    /// When `true`, trying to hover the content will result in the tooltip closing as the pointer leaves the trigger. Defaults to `false`.
    #[prop_or_default]
    pub disable_hoverable_content: Option<bool>,

    // Props from `TooltipPortalPrimitive`
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop_or_default]
    pub force_mount: Option<bool>,
    /// Specify a container element to portal the content into.
    #[prop_or_default]
    pub container: Option<web_sys::Element>,
    /// Specify a container element to portal the content into.
    #[prop_or_default]
    pub container_ref: Option<NodeRef>,

    // Props from `TooltipContentPrimitive`
    #[prop_or(TooltipSide::Top)]
    pub side: TooltipSide,
    #[prop_or(4.0)]
    pub side_offset: f64,
    #[prop_or(TooltipAlign::Center)]
    pub align: TooltipAlign,
    #[prop_or(0.0)]
    pub align_offset: f64,
    #[prop_or(0.0)]
    pub arrow_padding: f64,
    #[prop_or(true)]
    pub avoid_collisions: bool,
    #[prop_or_default]
    pub collision_boundary: Vec<web_sys::Element>,
    #[prop_or(TooltipPadding::All(10.0))]
    pub collision_padding: TooltipPadding,
    #[prop_or(TooltipSticky::Partial)]
    pub sticky: TooltipSticky,
    #[prop_or(false)]
    pub hide_when_detached: bool,
    #[prop_or(TooltipUpdatePositionStrategy::Optimized)]
    pub update_position_strategy: TooltipUpdatePositionStrategy,

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
    pub as_child: Option<Callback<TooltipChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type TooltipChildProps = TooltipTriggerPrimitiveChildProps;

#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let (class, style) = extract_props(
        &[&props.width, &props.min_width, &props.max_width],
        props.class.clone(),
        props.style.clone().into(),
    );

    html! {
        <TooltipPrimitive
            open={props.open}
            default_open={props.default_open}
            on_open_change={props.on_open_change.clone()}
            delay_duration={props.delay_duration}
            disable_hoverable_content={props.disable_hoverable_content}
        >
            // TODO: Automatically pass attributes/listeners if children is a single tag?
            <TooltipTriggerPrimitive as_child={props.as_child.clone()}>
                {props.children.clone()}
            </TooltipTriggerPrimitive>
            <TooltipPortalPrimitive
                force_mount={props.force_mount}
                container={props.container.clone()}
                container_ref={props.container_ref.clone()}
            >
                <Theme
                    class={class.to_string()}
                    id={props.id.clone()}
                    style={style}

                    node_ref={props.node_ref.clone()}
                    attributes={props.attributes.clone()}
                    as_child={Callback::from({
                        let content = props.content.clone();

                        let side = props.side;
                        let side_offset = props.side_offset;
                        let align = props.align;
                        let align_offset = props.align_offset;
                        let arrow_padding = props.arrow_padding;
                        let avoid_collisions = props.avoid_collisions;
                        let collision_boundary = props.collision_boundary.clone();
                        let collision_padding = props.collision_padding.clone();
                        let sticky = props.sticky;
                        let hide_when_detached = props.hide_when_detached;
                        let update_position_strategy = props.update_position_strategy;

                        move |ThemeChildProps {
                            node_ref,
                            attributes,

                            class,
                            data_accent_color,
                            data_gray_color,
                            data_has_background,
                            data_is_root_theme,
                            data_panel_background,
                            data_radius,
                            data_scaling,
                            id,
                            style,
                        }| html! {
                            <TooltipContentPrimitive
                                side={side}
                                side_offset={side_offset}
                                align={align}
                                align_offset={align_offset}
                                arrow_padding={arrow_padding}
                                avoid_collisions={avoid_collisions}
                                collision_boundary={collision_boundary.clone()}
                                collision_padding={collision_padding.clone()}
                                sticky={sticky}
                                hide_when_detached={hide_when_detached}
                                update_position_strategy={update_position_strategy}

                                class={classes!("rt-TooltipContent", &class).to_string()}
                                id={id}
                                style={style}

                                node_ref={node_ref}
                                attributes={attributes.with_defaults([
                                    ("data-accent-color", data_accent_color),
                                    ("data-gray-color", data_gray_color),
                                    ("data-has-background", data_has_background),
                                    ("data-is-root-theme", data_is_root_theme),
                                    ("data-panel-background", data_panel_background),
                                    ("data-radius", data_radius),
                                    ("data-scaling", data_scaling),
                                ])}
                            >
                                <Text r#as={TextAs::P} class="rt-TooltipText" size=1>{content.clone()}</Text>
                                <TooltipArrowPrimitive class="rt-TooltipArrow" />
                            </TooltipContentPrimitive>
                        }
                    })}
                />
            </TooltipPortalPrimitive>
        </TooltipPrimitive>
    }
}
