use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct CornerBottomRightIconProps {
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
pub fn CornerBottomRightIcon(props: &CornerBottomRightIconProps) -> Html {
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
                d="M5.12263 12H5.1H3.5C3.22386 12 3 11.7761 3 11.5C3 11.2239 3.22386 11 3.5 11H5.1C6.22836 11 7.04455 10.9996 7.68648 10.9472C8.32256 10.8952 8.74338 10.7946 9.08897 10.6185C9.74753 10.283 10.283 9.74753 10.6185 9.08897C10.7946 8.74338 10.8952 8.32256 10.9472 7.68648C10.9996 7.04455 11 6.22836 11 5.1V3.5C11 3.22386 11.2239 3 11.5 3C11.7761 3 12 3.22386 12 3.5V5.1V5.12263C12 6.22359 12 7.08052 11.9438 7.76791C11.8868 8.46584 11.7694 9.0329 11.5095 9.54296C11.0781 10.3897 10.3897 11.0781 9.54296 11.5095C9.0329 11.7694 8.46584 11.8868 7.76791 11.9438C7.08052 12 6.22359 12 5.12263 12Z"
                fill={& props.color}
            />
        </svg>
    }
}
