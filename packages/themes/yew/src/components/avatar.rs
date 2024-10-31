use radix_yew_avatar::{
    Avatar as AvatarPrimitive, AvatarChildProps as AvatarPrimitiveChildProps,
    AvatarFallback as AvatarFallbackPrimitive, AvatarImage as AvatarImagePrimitive,
    ImageLoadingStatus, SetAvatarChildProps as SetAvatarPrimitiveChildProps,
};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    components::avatar_props::{AvatarSizeProp, AvatarVariantProp},
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{
        color_prop::AccentColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct AvatarProps {
    #[prop_or_default]
    pub size: AvatarSizeProp,
    #[prop_or_default]
    pub variant: AvatarVariantProp,
    #[prop_or_default]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
    #[prop_or_default]
    pub radius: RadiusProp,
    pub fallback: Html,
    #[prop_or_default]
    pub m: MProp,
    #[prop_or_default]
    pub mx: MxProp,
    #[prop_or_default]
    pub my: MyProp,
    #[prop_or_default]
    pub mt: MtProp,
    #[prop_or_default]
    pub mr: MrProp,
    #[prop_or_default]
    pub mb: MbProp,
    #[prop_or_default]
    pub ml: MlProp,

    // Props from `AvatarImagePrimitive`
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
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub as_child: Option<Callback<AvatarChildProps, Html>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct AvatarChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub data_accent_color: String,
    pub data_radius: Option<String>,
}

impl SetAvatarPrimitiveChildProps for AvatarChildProps {
    fn set_avatar_child_props(&mut self, props: AvatarPrimitiveChildProps) {
        self.node_ref = props.node_ref;
        self.id = props.id;
        self.class = props.class;
        self.style = props.style;
    }
}

#[function_component]
pub fn Avatar(props: &AvatarProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
            &props.color,
            &props.high_contrast,
            &props.radius,
            &props.m,
            &props.mx,
            &props.my,
            &props.mt,
            &props.mr,
            &props.mb,
            &props.ml,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = AvatarChildProps {
        data_accent_color: props
            .color
            .0
            .map(|radius| radius.to_string())
            .unwrap_or_default(),
        data_radius: props.radius.0.map(|radius| radius.to_string()),
        ..AvatarChildProps::default()
    };

    html! {
        <AvatarPrimitive<AvatarChildProps>
            id={props.id.clone()}
            class={merge_classes(&[&"rt-reset", &"rt-AvatarRoot", &class])}
            style={style.to_string()}
            // TODO: `data-accent-color` and `data-radius` attributes
            as_child={props.as_child.clone()}
            as_child_props={child_props}
        >
            <AvatarImpl
                node_ref={props.node_ref.clone()}
                fallback={props.fallback.clone()}
                alt={props.alt.clone()}
                crossorigin={props.crossorigin.clone()}
                decoding={props.decoding.clone()}
                fetchpriority={props.fetchpriority.clone()}
                height={props.height.clone()}
                ismap={props.ismap}
                loading={props.loading.clone()}
                referrerpolicy={props.referrerpolicy.clone()}
                sizes={props.sizes.clone()}
                src={props.src.clone()}
                srcset={props.srcset.clone()}
                width={props.width.clone()}
                usemap={props.usemap.clone()}
            />
        </AvatarPrimitive<AvatarChildProps>>
    }
}

#[derive(PartialEq, Properties)]
struct AvatarImplProps {
    pub fallback: Html,

    // Props from `AvatarImagePrimitive`
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
    pub on_loading_status_change: Callback<ImageLoadingStatus>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
fn AvatarImpl(props: &AvatarImplProps) -> Html {
    let status = use_state_eq(|| ImageLoadingStatus::Idle);

    let on_loading_status_change = use_callback(props.on_loading_status_change.clone(), {
        let status = status.clone();

        move |value: ImageLoadingStatus, on_loading_status_change| {
            on_loading_status_change.emit(value);
            status.set(value);
        }
    });

    html! {
        <>
            if *status == ImageLoadingStatus::Idle || *status == ImageLoadingStatus::Loading {
                <span class="rt-AvatarFallback" />
            }

            if *status == ImageLoadingStatus::Error {
                <AvatarFallbackPrimitive
                    class={merge_classes(&[&"rt-AvatarFallback", &match &props.fallback {
                        VNode::VText(text) => match text.text.chars().count() {
                            1 => Some("rt-one-letter"),
                            2 => Some("rt-two-letters"),
                            _ => None
                        },
                        _ => None
                    }])}
                    delay_ms={0}
                >
                    {props.fallback.clone()}
                </AvatarFallbackPrimitive>
            }

            <AvatarImagePrimitive
                node_ref={props.node_ref.clone()}
                class="rt-AvatarImage"

                alt={props.alt.clone()}
                crossorigin={props.crossorigin.clone()}
                decoding={props.decoding.clone()}
                fetchpriority={props.fetchpriority.clone()}
                height={props.height.clone()}
                ismap={props.ismap}
                loading={props.loading.clone()}
                referrerpolicy={props.referrerpolicy.clone()}
                sizes={props.sizes.clone()}
                src={props.src.clone()}
                srcset={props.srcset.clone()}
                width={props.width.clone()}
                usemap={props.usemap.clone()}
                on_loading_status_change={on_loading_status_change}
            />
        </>
    }
}
