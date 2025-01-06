use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BorderWidthIconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn BorderWidthIcon(props: BorderWidthIconProps) -> Element {
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.width}",
            "height": "{props.height}",
            "viewBox": "0 0 15 15",
            "fill": "none",
            path {
                "fill-rule": "evenodd",
                "clip-rule": "evenodd",
                "d": "M1 3H14V4H1V3ZM1 6H14V8H1V6ZM14 10.25H1V12.75H14V10.25Z",
                "fill": "{props.color}",
            }
        }
    }
}
