use std::fmt::Display;

use yew::prelude::*;

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
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SeparatorChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SeparatorChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub data_orientation: String,
    pub aria_orientation: Option<String>,
    pub role: String,
}

impl SeparatorChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                data-orientation={self.data_orientation}
                aria-orientation={self.aria_orientation}
                role={self.role}
            >
                {children}
            </div>
        }
    }
}

#[function_component]
pub fn Separator(props: &SeparatorProps) -> Html {
    let aria_orientation = match props.orientation {
        Orientation::Horizontal => None,
        Orientation::Vertical => Some("vertical".to_string()),
    };

    let child_props = SeparatorChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        data_orientation: props.orientation.to_string(),
        aria_orientation: match props.decorative {
            true => aria_orientation,
            false => None,
        },
        role: match props.decorative {
            true => "none",
            false => "separator",
        }
        .into(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
