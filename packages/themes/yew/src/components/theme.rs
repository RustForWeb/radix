use radix_yew_direction::{Direction, DirectionProvider};
use radix_yew_tooltip::TooltipProvider;
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::theme_props::{Appearance, PanelBackground, Scaling},
    helpers::get_matching_gray_color::get_matching_gray_color,
    props::{
        color_prop::{AccentColor, GrayColor},
        radius_prop::Radius,
    },
};

#[derive(Clone, PartialEq)]
pub struct ThemeContextValue {
    pub appearance: Appearance,
    pub accent_color: AccentColor,
    pub gray_color: GrayColor,
    pub resolved_gray_color: GrayColor,
    pub panel_background: PanelBackground,
    pub radius: Radius,
    pub scaling: Scaling,
    pub on_appearance_change: Callback<Appearance>,
    pub on_accent_color_change: Callback<AccentColor>,
    pub on_gray_color_change: Callback<GrayColor>,
    pub on_panel_background_change: Callback<PanelBackground>,
    pub on_radius_change: Callback<Radius>,
    pub on_scaling_change: Callback<Scaling>,
}

#[hook]
pub fn use_theme_context() -> ThemeContextValue {
    use_context::<ThemeContextValue>().expect("`use_theme_context` must be used within a `Theme`")
}

#[derive(PartialEq, Properties)]
pub struct ThemeProps {
    #[prop_or_default]
    pub has_background: Option<bool>,
    #[prop_or_default]
    pub appearance: Option<Appearance>,
    #[prop_or_default]
    pub accent_color: Option<AccentColor>,
    #[prop_or_default]
    pub gray_color: Option<GrayColor>,
    #[prop_or_default]
    pub panel_background: Option<PanelBackground>,
    #[prop_or_default]
    pub radius: Option<Radius>,
    #[prop_or_default]
    pub scaling: Option<Scaling>,

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
    pub as_child: Option<Callback<ThemeChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Theme(props: &ThemeProps) -> Html {
    let context = use_context::<ThemeContextValue>();
    let is_root = context.is_none();

    html! {
        if is_root {
            <TooltipProvider delay_duration=200>
                <DirectionProvider direction={Direction::Ltr}>
                    <ThemeRoot
                        has_background={props.has_background}
                        appearance={props.appearance}
                        accent_color={props.accent_color}
                        gray_color={props.gray_color}
                        panel_background={props.panel_background}
                        radius={props.radius}
                        scaling={props.scaling}

                        class={props.class.clone()}
                        id={props.id.clone()}
                        style={props.style.clone()}

                        node_ref={props.node_ref.clone()}
                        attributes={props.attributes.clone()}
                        as_child={props.as_child.clone()}
                    >
                        {props.children.clone()}
                    </ThemeRoot>
                </DirectionProvider>
            </TooltipProvider>
        } else {
            <ThemeImpl
                has_background={props.has_background}
                appearance={props.appearance}
                accent_color={props.accent_color}
                gray_color={props.gray_color}
                panel_background={props.panel_background}
                radius={props.radius}
                scaling={props.scaling}

                class={props.class.clone()}
                id={props.id.clone()}
                style={props.style.clone()}

                node_ref={props.node_ref.clone()}
                attributes={props.attributes.clone()}
                as_child={props.as_child.clone()}
            >
                {props.children.clone()}
            </ThemeImpl>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemeRootProps {
    #[prop_or_default]
    pub has_background: Option<bool>,
    #[prop_or_default]
    pub appearance: Option<Appearance>,
    #[prop_or_default]
    pub accent_color: Option<AccentColor>,
    #[prop_or_default]
    pub gray_color: Option<GrayColor>,
    #[prop_or_default]
    pub panel_background: Option<PanelBackground>,
    #[prop_or_default]
    pub radius: Option<Radius>,
    #[prop_or_default]
    pub scaling: Option<Scaling>,

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
    pub as_child: Option<Callback<ThemeChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ThemeRoot(props: &ThemeRootProps) -> Html {
    let appearance_prop = props.appearance.unwrap_or_default();
    let accent_color_prop = props.accent_color.unwrap_or_default();
    let gray_color_prop = props.gray_color.unwrap_or_default();
    let panel_background_prop = props.panel_background.unwrap_or_default();
    let radius_prop = props.radius.unwrap_or_default();
    let scaling_prop = props.scaling.unwrap_or_default();
    let has_background = props.has_background.unwrap_or(true);

    let appearance = use_state_eq(|| appearance_prop);
    use_effect_with(appearance_prop, {
        let appearance = appearance.clone();

        move |appearance_prop| appearance.set(*appearance_prop)
    });

    let accent_color = use_state_eq(|| accent_color_prop);
    use_effect_with(accent_color_prop, {
        let accent_color = accent_color.clone();

        move |accent_color_prop| accent_color.set(*accent_color_prop)
    });

    let gray_color = use_state_eq(|| gray_color_prop);
    use_effect_with(gray_color_prop, {
        let gray_color = gray_color.clone();

        move |gray_color_prop| gray_color.set(*gray_color_prop)
    });

    let panel_background = use_state_eq(|| panel_background_prop);
    use_effect_with(panel_background_prop, {
        let panel_background = panel_background.clone();

        move |panel_background_prop| panel_background.set(*panel_background_prop)
    });

    let radius = use_state_eq(|| radius_prop);
    use_effect_with(radius_prop, {
        let radius = radius.clone();

        move |radius_prop| radius.set(*radius_prop)
    });

    let scaling = use_state_eq(|| scaling_prop);
    use_effect_with(scaling_prop, {
        let scaling = scaling.clone();

        move |scaling_prop| scaling.set(*scaling_prop)
    });

    html! {
        <ThemeImpl
            is_root=true
            has_background={has_background}

            appearance={*appearance}
            accent_color={*accent_color}
            gray_color={*gray_color}
            panel_background={*panel_background}
            radius={*radius}
            scaling={*scaling}

            on_appearance_change={Callback::from(move |value| appearance.set(value))}
            on_accent_color_change={Callback::from(move |value| accent_color.set(value))}
            on_gray_color_change={Callback::from(move |value| gray_color.set(value))}
            on_panel_background_change={Callback::from(move |value| panel_background.set(value))}
            on_radius_change={Callback::from(move |value| radius.set(value))}
            on_scaling_change={Callback::from(move |value| scaling.set(value))}

            class={props.class.clone()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
        </ThemeImpl>
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemeImplProps {
    #[prop_or(false)]
    pub is_root: bool,
    #[prop_or_default]
    pub has_background: Option<bool>,
    #[prop_or_default]
    pub appearance: Option<Appearance>,
    #[prop_or_default]
    pub accent_color: Option<AccentColor>,
    #[prop_or_default]
    pub gray_color: Option<GrayColor>,
    #[prop_or_default]
    pub panel_background: Option<PanelBackground>,
    #[prop_or_default]
    pub radius: Option<Radius>,
    #[prop_or_default]
    pub scaling: Option<Scaling>,
    #[prop_or_default]
    pub on_appearance_change: Callback<Appearance>,
    #[prop_or_default]
    pub on_accent_color_change: Callback<AccentColor>,
    #[prop_or_default]
    pub on_gray_color_change: Callback<GrayColor>,
    #[prop_or_default]
    pub on_panel_background_change: Callback<PanelBackground>,
    #[prop_or_default]
    pub on_radius_change: Callback<Radius>,
    #[prop_or_default]
    pub on_scaling_change: Callback<Scaling>,

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
    pub as_child: Option<Callback<ThemeChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct ThemeChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: String,
    pub data_gray_color: String,
    pub data_has_background: String,
    pub data_is_root_theme: String,
    pub data_panel_background: String,
    pub data_radius: String,
    pub data_scaling: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn ThemeImpl(props: &ThemeImplProps) -> Html {
    let context = use_context::<ThemeContextValue>();
    let is_root = props.is_root;
    let appearance = props
        .appearance
        .or(context.as_ref().map(|context| context.appearance))
        .unwrap_or_default();
    let accent_color = props
        .accent_color
        .or(context.as_ref().map(|context| context.accent_color))
        .unwrap_or_default();
    let gray_color = props
        .gray_color
        .or(context.as_ref().map(|context| context.resolved_gray_color))
        .unwrap_or_default();
    let panel_background = props
        .panel_background
        .or(context.as_ref().map(|context| context.panel_background))
        .unwrap_or_default();
    let radius = props
        .radius
        .or(context.as_ref().map(|context| context.radius))
        .unwrap_or_default();
    let scaling = props
        .scaling
        .or(context.as_ref().map(|context| context.scaling))
        .unwrap_or_default();
    let resolved_gray_color = match gray_color {
        GrayColor::Auto => get_matching_gray_color(accent_color),
        gray_color => gray_color,
    };
    let is_explicit_appearance = matches!(
        props.appearance,
        Some(Appearance::Light) | Some(Appearance::Dark)
    );
    let has_background = match props.has_background {
        Some(has_background) => has_background,
        None => is_root || is_explicit_appearance,
    };

    #[derive(PartialEq)]
    struct ContextValueMemo {
        appearance: Appearance,
        accent_color: AccentColor,
        gray_color: GrayColor,
        resolved_gray_color: GrayColor,
        panel_background: PanelBackground,
        radius: Radius,
        scaling: Scaling,
        on_appearance_change: Callback<Appearance>,
        on_accent_color_change: Callback<AccentColor>,
        on_gray_color_change: Callback<GrayColor>,
        on_panel_background_change: Callback<PanelBackground>,
        on_radius_change: Callback<Radius>,
        on_scaling_change: Callback<Scaling>,
    }

    let context_value = use_memo(
        ContextValueMemo {
            appearance,
            accent_color,
            gray_color,
            resolved_gray_color,
            panel_background,
            radius,
            scaling,
            on_appearance_change: props.on_appearance_change.clone(),
            on_accent_color_change: props.on_accent_color_change.clone(),
            on_gray_color_change: props.on_gray_color_change.clone(),
            on_panel_background_change: props.on_panel_background_change.clone(),
            on_radius_change: props.on_radius_change.clone(),
            on_scaling_change: props.on_scaling_change.clone(),
        },
        |ContextValueMemo {
             appearance,
             accent_color,
             gray_color,
             resolved_gray_color,
             panel_background,
             radius,
             scaling,
             on_appearance_change,
             on_accent_color_change,
             on_gray_color_change,
             on_panel_background_change,
             on_radius_change,
             on_scaling_change,
         }| ThemeContextValue {
            appearance: *appearance,
            accent_color: *accent_color,
            gray_color: *gray_color,
            resolved_gray_color: *resolved_gray_color,
            panel_background: *panel_background,
            radius: *radius,
            scaling: *scaling,
            on_appearance_change: on_appearance_change.clone(),
            on_accent_color_change: on_accent_color_change.clone(),
            on_gray_color_change: on_gray_color_change.clone(),
            on_panel_background_change: on_panel_background_change.clone(),
            on_radius_change: on_radius_change.clone(),
            on_scaling_change: on_scaling_change.clone(),
        },
    );

    let child_props = ThemeChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!(
            "radix-themes",
            match appearance {
                Appearance::Inherit => None,
                Appearance::Light => Some("light"),
                Appearance::Dark => Some("dark"),
            },
            &props.class,
        )
        .to_string(),
        data_accent_color: accent_color.to_string(),
        data_gray_color: resolved_gray_color.to_string(),
        // For nested `Theme` background.
        data_has_background: if has_background { "true" } else { "false" }.to_owned(),
        data_is_root_theme: if is_root { "true" } else { "false" }.to_owned(),
        data_panel_background: panel_background.to_string(),
        data_radius: radius.to_string(),
        data_scaling: scaling.to_string(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    html! {
        <ContextProvider<ThemeContextValue> context={(*context_value).clone()}>
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        </ContextProvider<ThemeContextValue>>
    }
}
