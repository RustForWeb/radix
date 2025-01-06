use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BorderRightIconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn BorderRightIcon(props: BorderRightIconProps) -> Element {
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
                "d": "M13.25 1L13.25 14L14.75 14L14.75 1L13.25 1Z",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 5 7)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 5 13)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 3 7)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 3 13)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 7 7)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 1 7)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 7 13)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 1 13)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 7 5)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 1 5)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 7 3)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 1 3)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 7 9)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 1 9)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 7 11)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 1 11)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 9 7)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 9 13)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 11 7)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 11 13)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 5 1)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 3 1)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 7 1)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 1 1)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 9 1)",
                "fill": "{props.color}",
            }
            rect {
                "width": "1",
                "height": "1",
                "rx": ".5",
                "transform": "matrix(0 1 1 0 11 1)",
                "fill": "{props.color}",
            }
        }
    }
}
