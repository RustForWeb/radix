use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TriangleRightIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component(UseNodeRef)]
pub fn TriangleRightIcon(props: &TriangleRightIconProps) -> Html {
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
            <path d="M6 11L6 4L10.5 7.5L6 11Z" fill={& props.color} />
        </svg>
    }
}
