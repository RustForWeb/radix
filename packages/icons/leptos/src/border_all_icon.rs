use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BorderAllIcon(
    #[prop(default = 15.into(), into)] width: Signal<usize>,
    #[prop(default = 15.into(), into)] height: Signal<usize>,
    #[prop(default = "currentColor".into(), into)] color: Signal<String>,
    #[prop(optional)] node_ref: NodeRef<Svg>,
) -> impl IntoView {
    view! {
        <svg
            node_ref=node_ref
            width=width
            height=height
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M0.25 1C0.25 0.585786 0.585786 0.25 1 0.25H14C14.4142 0.25 14.75 0.585786 14.75 1V14C14.75 14.4142 14.4142 14.75 14 14.75H1C0.585786 14.75 0.25 14.4142 0.25 14V1ZM1.75 1.75V13.25H13.25V1.75H1.75Z"
                fill=color
            ></path>
            <rect x="7" y="5" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="3" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="5" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="3" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="9" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="11" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="9" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="11" width="1" height="1" rx=".5" fill=color></rect>
        </svg>
    }
}
