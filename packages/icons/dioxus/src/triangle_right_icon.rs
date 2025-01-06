use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct TriangleRightIconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn TriangleRightIcon(props: TriangleRightIconProps) -> Element {
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.width}",
            "height": "{props.height}",
            "viewBox": "0 0 15 15",
            "fill": "none",
            path { "d": "M6 11L6 4L10.5 7.5L6 11Z", "fill": "{props.color}" }
        }
    }
}
