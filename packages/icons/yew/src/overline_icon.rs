use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct OverlineIconProps {
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
pub fn OverlineIcon(props: &OverlineIconProps) -> Html {
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
                d="M3.49985 1.10001C3.27894 1.10001 3.09985 1.27909 3.09985 1.50001C3.09985 1.72092 3.27894 1.90001 3.49985 1.90001H11.4999C11.7208 1.90001 11.8999 1.72092 11.8999 1.50001C11.8999 1.27909 11.7208 1.10001 11.4999 1.10001H3.49985ZM4.99995 4.25001C4.99995 3.97387 4.77609 3.75001 4.49995 3.75001C4.22381 3.75001 3.99995 3.97387 3.99995 4.25001V9.55001C3.99995 11.483 5.56695 13.05 7.49995 13.05C9.43295 13.05 11 11.483 11 9.55001V4.25001C11 3.97387 10.7761 3.75001 10.5 3.75001C10.2238 3.75001 9.99995 3.97387 9.99995 4.25001V9.55001C9.99995 10.9307 8.88066 12.05 7.49995 12.05C6.11924 12.05 4.99995 10.9307 4.99995 9.55001V4.25001Z"
                fill={& props.color}
            />
        </svg>
    }
}
