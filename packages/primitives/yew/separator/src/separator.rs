use std::fmt::Display;

use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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
    pub as_child: Option<Callback<SeparatorChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct SeparatorChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub aria_orientation: Option<String>,
    pub class: Option<String>,
    pub data_orientation: String,
    pub id: Option<String>,
    pub role: String,
    pub style: Style,
}

#[function_component]
pub fn Separator(props: &SeparatorProps) -> Html {
    let aria_orientation = match props.orientation {
        Orientation::Horizontal => None,
        Orientation::Vertical => Some("vertical".to_owned()),
    };

    let child_props = SeparatorChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        aria_orientation: match props.decorative {
            true => aria_orientation,
            false => None,
        },
        class: props.class.clone(),
        data_orientation: props.orientation.to_string(),
        id: props.id.clone(),
        role: match props.decorative {
            true => "none",
            false => "separator",
        }
        .to_owned(),
        style: props.style.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
