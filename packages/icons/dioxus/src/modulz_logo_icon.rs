use dioxus::prelude::*;
#[derive(Clone, PartialEq, Props)]
pub struct ModulzLogoIconProps {
    #[props(default = 15)]
    pub width: usize,
    #[props(default = 15)]
    pub height: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    pub class: Option<String>,
}
#[component]
pub fn ModulzLogoIcon(props: ModulzLogoIconProps) -> Element {
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
                "d": "M7.25925 3.16667L4.37036 5.33333V1L7.25925 3.16667ZM1 8.22222L3.88889 6.05555L1 3.88889V8.22222ZM1 14L3.88889 11.8333L1 9.66666V14ZM7.74072 8.22222L10.6296 6.05555L7.74072 3.88889V8.22222ZM14 3.16667L11.1111 5.33333V1L14 3.16667ZM11.1111 11.1111L14 8.94444L11.1111 6.77777V11.1111ZM3.88889 11.1111L1 8.94444L3.88889 6.77777V11.1111ZM4.37036 6.05555L7.25925 8.22222V3.88889L4.37036 6.05555ZM3.88889 5.33333L1 3.16667L3.88889 1V5.33333ZM7.74072 3.16667L10.6296 5.33333V1L7.74072 3.16667ZM14 8.22222L11.1111 6.05555L14 3.88889V8.22222ZM11.1111 11.8333L14 14V9.66666L11.1111 11.8333Z",
                "fill": "{props.color}",
            }
        }
    }
}
