use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct DragHandleDots1IconProps {
    #[prop_or(15)]
    pub width: usize,
    #[prop_or(15)]
    pub height: usize,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub node_ref: NodeRef,
}
#[function_component]
pub fn DragHandleDots1Icon(props: &DragHandleDots1IconProps) -> Html {
    html! {
        <svg
            ref={props.node_ref.clone()}
            class={props.class.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle cx="4.5" cy="2.5" r=".6" fill={& props.color} />
            <circle cx="4.5" cy="4.5" r=".6" fill={& props.color} />
            <circle cx="4.5" cy="6.499" r=".6" fill={& props.color} />
            <circle cx="4.5" cy="8.499" r=".6" fill={& props.color} />
            <circle
                cx="4.5"
                cy="10.498"
                r=".6"
                fill={& props
        .color}
            />
            <circle cx="4.5" cy="12.498" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="2.5" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="4.5" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="6.499" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="8.499" r=".6" fill={& props.color} />
            <circle cx="6.5" cy="10.498" r=".6" fill={& props.color} />
            <circle
                cx="6.5"
                cy="12.498"
                r=".6"
                fill={&
        props.color}
            />
            <circle cx="8.499" cy="2.5" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="4.5" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="6.499" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="8.499" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="10.498" r=".6" fill={& props.color} />
            <circle cx="8.499" cy="12.498" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="2.5" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="4.5" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="6.499" r=".6" fill={& props.color} />
            <circle
                cx="10.499"
                cy="8.499"
                r=".6"
                fill={& props
        .color}
            />
            <circle cx="10.499" cy="10.498" r=".6" fill={& props.color} />
            <circle cx="10.499" cy="12.498" r=".6" fill={& props.color} />
        </svg>
    }
}
