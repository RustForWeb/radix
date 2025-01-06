use leptos::{prelude::*, svg::Svg};
#[component]
pub fn BorderBottomIcon(
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
                d="M1 13.25L14 13.25V14.75L1 14.75V13.25Z"
                fill=color
            ></path>
            <rect x="7" y="5" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="13" y="5" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="3" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="13" y="3" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="1" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="13" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="13" y="1" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="5" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="5" y="1" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="3" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="3" y="1" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="9" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="9" y="1" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="11" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="11" y="1" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="9" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="13" y="9" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="7" y="11" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="13" y="11" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="1" y="5" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="1" y="3" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="1" y="7" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="1" y="1" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="1" y="9" width="1" height="1" rx=".5" fill=color></rect>
            <rect x="1" y="11" width="1" height="1" rx=".5" fill=color></rect>
        </svg>
    }
}
