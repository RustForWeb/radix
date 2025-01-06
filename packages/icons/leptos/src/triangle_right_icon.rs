use leptos::{prelude::*, svg::Svg};
#[component]
pub fn TriangleRightIcon(
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
            <path d="M6 11L6 4L10.5 7.5L6 11Z" fill=color></path>
        </svg>
    }
}
