use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SwitchIconProps {
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
pub fn SwitchIcon(props: &SwitchIconProps) -> Html {
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
                d="M10.5 4C8.567 4 7 5.567 7 7.5C7 9.433 8.567 11 10.5 11C12.433 11 14 9.433 14 7.5C14 5.567 12.433 4 10.5 4ZM7.67133 11C6.65183 10.175 6 8.91363 6 7.5C6 6.08637 6.65183 4.82498 7.67133 4H4.5C2.567 4 1 5.567 1 7.5C1 9.433 2.567 11 4.5 11H7.67133ZM0 7.5C0 5.01472 2.01472 3 4.5 3H10.5C12.9853 3 15 5.01472 15 7.5C15 9.98528 12.9853 12 10.5 12H4.5C2.01472 12 0 9.98528 0 7.5Z"
                fill={& props.color}
            />
        </svg>
    }
}
