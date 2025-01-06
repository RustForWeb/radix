use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct BorderNoneIconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn BorderNoneIcon(props: BorderNoneIconProps) -> Element {
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "width": "{props.width}",
            "height": "{props.height}",
            "viewBox": "0 0 15 15",
            "fill": "none",
            rect {
                "x": "7",
                "y": "5.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "13",
                "y": "5.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "7",
                "y": "3.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "13",
                "y": "3.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "7",
                "y": "7.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "7",
                "y": "13.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "7",
                "y": "1.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "13",
                "y": "7.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "13",
                "y": "13.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "13",
                "y": "1.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "5",
                "y": "7.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "5",
                "y": "13.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "5",
                "y": "1.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "3",
                "y": "7.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "3",
                "y": "13.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "3",
                "y": "1.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "9",
                "y": "7.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "9",
                "y": "13.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "9",
                "y": "1.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "11",
                "y": "7.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "11",
                "y": "13.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "11",
                "y": "1.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "7",
                "y": "9.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "13",
                "y": "9.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "7",
                "y": "11.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "13",
                "y": "11.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "1",
                "y": "5.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "1",
                "y": "3.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "1",
                "y": "7.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "1",
                "y": "13.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "1",
                "y": "1.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "1",
                "y": "9.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
            rect {
                "x": "1",
                "y": "11.025",
                "width": "1",
                "height": "1",
                "rx": ".5",
                "fill": "{props.color}",
            }
        }
    }
}
