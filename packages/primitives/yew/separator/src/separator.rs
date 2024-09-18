use std::fmt::Display;

use radix_yew_primitive::Primitive;
use yew::prelude::*;
use yew_attrs::{attrs, Attrs};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
        )
    }
}

#[derive(PartialEq, Properties)]
pub struct SeparatorProps {
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or(false)]
    pub decorative: bool,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Separator(props: &SeparatorProps) -> Html {
    let aria_orientation = match props.orientation {
        Orientation::Horizontal => None,
        Orientation::Vertical => Some("vertical".to_string()),
    };

    let attrs = props
        .attrs
        .clone()
        .merge(attrs! {
            data-orientation={props.orientation.to_string()}
            aria-orientation={match props.decorative {
                true => aria_orientation,
                false => None,
            }}
            role={match props.decorative {
                true => "none",
                false => "separator"
            }}
        })
        .expect("Attributes should be merged.");

    html! {
        <Primitive
            element="div"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={attrs}
        >
            {props.children.clone()}
        </Primitive>
    }
}
