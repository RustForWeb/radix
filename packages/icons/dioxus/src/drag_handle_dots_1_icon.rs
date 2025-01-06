use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct DragHandleDots1IconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn DragHandleDots1Icon(props: DragHandleDots1IconProps) -> Element {
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.width}",
            "height": "{props.height}",
            "viewBox": "0 0 15 15",
            "fill": "none",
            circle {
                "cx": "4.5",
                "cy": "2.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "4.5",
                "cy": "4.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "4.5",
                "cy": "6.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "4.5",
                "cy": "8.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "4.5",
                "cy": "10.498",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "4.5",
                "cy": "12.498",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "6.5",
                "cy": "2.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "6.5",
                "cy": "4.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "6.5",
                "cy": "6.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "6.5",
                "cy": "8.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "6.5",
                "cy": "10.498",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "6.5",
                "cy": "12.498",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "8.499",
                "cy": "2.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "8.499",
                "cy": "4.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "8.499",
                "cy": "6.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "8.499",
                "cy": "8.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "8.499",
                "cy": "10.498",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "8.499",
                "cy": "12.498",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "10.499",
                "cy": "2.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "10.499",
                "cy": "4.5",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "10.499",
                "cy": "6.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "10.499",
                "cy": "8.499",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "10.499",
                "cy": "10.498",
                "r": ".6",
                "fill": "{props.color}",
            }
            circle {
                "cx": "10.499",
                "cy": "12.498",
                "r": ".6",
                "fill": "{props.color}",
            }
        }
    }
}
