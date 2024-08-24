use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TriangleLeftIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component]
pub fn TriangleLeftIcon(props: &TriangleLeftIconProps) -> Html {
    let node_ref = use_node_ref();
    html! {
        <svg
            ref={node_ref}
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path d="M9 4L9 11L4.5 7.5L9 4Z" fill={& props.color} />
        </svg>
    }
}
