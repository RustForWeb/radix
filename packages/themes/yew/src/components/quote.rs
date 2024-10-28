use yew::prelude::*;

use crate::{
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{text_wrap_prop::TextWrapProp, truncate_prop::TruncateProp},
};

#[derive(PartialEq, Properties)]
pub struct QuoteProps {
    #[prop_or_default]
    pub truncate: TruncateProp,
    #[prop_or_default]
    pub wrap: TextWrapProp,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub as_child: Option<Callback<QuoteChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct QuoteChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Style,
}

impl QuoteChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <q
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style.to_string()}
            >
                {children}
            </q>
        }
    }
}

#[function_component]
pub fn Quote(props: &QuoteProps) -> Html {
    let (class, style) = extract_props(
        &[&props.truncate, &props.wrap],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = QuoteChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: merge_classes(&[&"rt-Quote", &class]),
        style,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
