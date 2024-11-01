use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BorderLeftIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub height: AttrValue,
}
#[function_component]
pub fn BorderLeftIcon(props: &BorderLeftIconProps) -> Html {
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
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1.75 1L1.75 14L0.249999 14L0.25 1L1.75 1Z"
                fill={& props.color}
            />
            <rect
                x="10"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 10 7)"
                fill={& props.color}
            />
            <rect
                x="10"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 10 13)"
                fill={& props.color}
            />
            <rect
                x="12"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 12 7)"
                fill={& props.color}
            />
            <rect
                x="12"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 12 13)"
                fill={& props.color}
            />
            <rect
                x="8"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 7)"
                fill={& props
        .color}
            />
            <rect
                x="14"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 7)"
                fill={& props.color}
            />
            <rect
                x="8"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 13)"
                fill={& props.color}
            />
            <rect
                x="14"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 13)"
                fill={& props.color}
            />
            <rect
                x="8"
                y="5"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 5)"
                fill={& props.color}
            />
            <rect
                x="14"
                y="5"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 5)"
                fill={& props.color}
            />
            <rect
                x="8"
                y="3"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 3)"
                fill={& props.color}
            />
            <rect
                x="14"
                y="3"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 3)"
                fill={& props.color}
            />
            <rect
                x="8"
                y="9"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 9)"
                fill={& props.color}
            />
            <rect
                x="14"
                y="9"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 9)"
                fill={& props.color}
            />
            <rect
                x="8"
                y="11"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 11)"
                fill={& props.color}
            />
            <rect
                x="14"
                y="11"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 11)"
                fill={& props.color}
            />
            <rect
                x="6"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 6 7)"
                fill={& props.color}
            />
            <rect
                x="6"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 6 13)"
                fill={& props.color}
            />
            <rect
                x="4"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 4 7)"
                fill={& props.color}
            />
            <rect
                x="4"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 4 13)"
                fill={& props.color}
            />
            <rect
                x="10"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 10 1)"
                fill={& props.color}
            />
            <rect
                x="12"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 12 1)"
                fill={& props.color}
            />
            <rect
                x="8"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 1)"
                fill={& props.color}
            />
            <rect
                x="14"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 1)"
                fill={& props.color}
            />
            <rect
                x="6"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 6 1)"
                fill={& props.color}
            />
            <rect
                x="4"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 4 1)"
                fill={& props.color}
            />
        </svg>
    }
}
