use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct AllSidesIconProps {
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
pub fn AllSidesIcon(props: &AllSidesIconProps) -> Html {
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
                d="M7.5 0.75L9.75 3H5.25L7.5 0.75ZM7.5 14.25L9.75 12H5.25L7.5 14.25ZM3 5.25L0.75 7.5L3 9.75V5.25ZM14.25 7.5L12 5.25V9.75L14.25 7.5Z"
                fill={& props.color}
            />
        </svg>
    }
}
