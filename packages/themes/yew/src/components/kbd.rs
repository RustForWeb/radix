use yew::prelude::*;

use crate::{
    components::kbd_props::KbdSizeProp,
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
};

#[derive(PartialEq, Properties)]
pub struct KbdProps {
    #[prop_or_default]
    pub size: KbdSizeProp,
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

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub as_child: Option<Callback<KbdChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct KbdChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Style,
}

impl KbdChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <kbd
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style.to_string()}
            >
                {children}
            </kbd>
        }
    }
}

#[function_component]
pub fn Kbd(props: &KbdProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
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

    let child_props = KbdChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: merge_classes(&[&"rt-reset", &"rt-Kbd", &class]),
        style,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
