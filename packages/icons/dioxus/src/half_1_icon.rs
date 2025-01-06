use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct Half1IconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn Half1Icon(props: Half1IconProps) -> Element {
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
                "d": "M7.49991 0.876892C3.84222 0.876892 0.877075 3.84204 0.877075 7.49972C0.877075 11.1574 3.84222 14.1226 7.49991 14.1226C11.1576 14.1226 14.1227 11.1574 14.1227 7.49972C14.1227 3.84204 11.1576 0.876892 7.49991 0.876892ZM7.00003 1.84861C4.10114 2.1017 1.82707 4.53515 1.82707 7.49972C1.82707 10.4643 4.10114 12.8977 7.00003 13.1508V1.84861ZM8.00003 13.1508C10.8988 12.8976 13.1727 10.4642 13.1727 7.49972C13.1727 4.53524 10.8988 2.10185 8.00003 1.84864V13.1508Z",
                "fill": "{props.color}",
            }
        }
    }
}
