use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ZoomOutIconProps {
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
pub fn ZoomOutIcon(props: &ZoomOutIconProps) -> Html {
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
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M6.5 10C8.433 10 10 8.433 10 6.5C10 4.567 8.433 3 6.5 3C4.567 3 3 4.567 3 6.5C3 8.433 4.567 10 6.5 10ZM6.5 11C7.56251 11 8.53901 10.6318 9.30884 10.0159L12.1464 12.8536C12.3417 13.0488 12.6583 13.0488 12.8536 12.8536C13.0488 12.6583 13.0488 12.3417 12.8536 12.1464L10.0159 9.30884C10.6318 8.53901 11 7.56251 11 6.5C11 4.01472 8.98528 2 6.5 2C4.01472 2 2 4.01472 2 6.5C2 8.98528 4.01472 11 6.5 11ZM4.75 6C4.47386 6 4.25 6.22386 4.25 6.5C4.25 6.77614 4.47386 7 4.75 7H8.25C8.52614 7 8.75 6.77614 8.75 6.5C8.75 6.22386 8.52614 6 8.25 6H4.75Z"
                fill={& props.color}
            />
        </svg>
    }
}
