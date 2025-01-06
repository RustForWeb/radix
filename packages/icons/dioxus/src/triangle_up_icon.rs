use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TriangleUpIconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn TriangleUpIcon(props: TriangleUpIconProps) -> Element {
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.width}",
            "height": "{props.height}",
            "viewBox": "0 0 15 15",
            "fill": "none",
            path { "d": "M4 9H11L7.5 4.5L4 9Z", "fill": "{props.color}" }
        }
    }
}
