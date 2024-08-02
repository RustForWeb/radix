use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TriangleUpIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component(UseNodeRef)]
pub fn TriangleUpIcon(props: &TriangleUpIconProps) -> Html {
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
            <path d="M4 9H11L7.5 4.5L4 9Z" fill={& props.color} />
        </svg>
    }
}
