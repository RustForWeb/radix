use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AspectRatioProps {
    #[prop_or(1.0)]
    pub ratio: f64,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<AspectRatioChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct AspectRatioChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: String,
}

impl AspectRatioChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
            >
                {children}
            </div>
        }
    }
}

#[function_component]
pub fn AspectRatio(props: &AspectRatioProps) -> Html {
    let child_props = AspectRatioChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        // Ensures children expand in ratio.
        style: format!(
            "position: absolute; top: 0px; right: 0px; bottom: 0px; left: 0px;{}",
            props.style.clone().unwrap_or_default()
        ),
    };

    html! {
        <div
            // Posittion ensures inner element is contained.
            // Width ensures padding bottom trick maths works.
            style={format!("position: relative; width: 100%; padding-bottom: {}%;", 100.0 / props.ratio)}
            data-radix-aspect-ratio-wrapper=""
        >
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        </div>
    }
}
