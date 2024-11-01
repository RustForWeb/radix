use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct DragHandleDots1IconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub height: AttrValue,
}
#[function_component]
pub fn DragHandleDots1Icon(props: &DragHandleDots1IconProps) -> Html {
    let node_ref = use_node_ref();
    html! {
        <svg
            ref={node_ref}
            width={& props.width}
            height={& props.height}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle cx="4.5" cy="2.5" r=".6" fill={& props.color} />
            <circle cx="4.5" cy="4.5" r=".6" fill={& props.color} />
            <circle cx="4.5" cy="6.499" r=".6" fill={& props.color} />
            <circle cx="4.5" cy="8.499" r=".6" fill={& props.color} />
            <circle cx="4.5" cy="10.498" r=".6" fill={& props.color} />
            <circle
                cx="4.5"
                cy="12.498"
                r=".6"
                fill={&
        props.color}
            />
            <circle cx="6.5" cy="2.5" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="4.5" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="6.499" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="8.499" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="10.498" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="12.498" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="2.5" r=".6" fill={& props.color} />
            <circle
                cx="8.499"
                cy="4.5"
                r=".6"
                fill={& props
        .color}
            />
            <circle cx="8.499" cy="6.499" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="8.499" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="10.498" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="12.498" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="2.5" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="4.5" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="6.499" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="8.499" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="10.498" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="12.498" r=".6" fill={& props.color} />
        </svg>
    }
}
