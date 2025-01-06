use leptos::{prelude::*, svg::Svg};
#[component]
pub fn DragHandleDots1Icon(
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
            <circle cx="4.5" cy="2.5" r=".6" fill=color></circle>
            <circle cx="4.5" cy="4.5" r=".6" fill=color></circle>
            <circle cx="4.5" cy="6.499" r=".6" fill=color></circle>
            <circle cx="4.5" cy="8.499" r=".6" fill=color></circle>
            <circle cx="4.5" cy="10.498" r=".6" fill=color></circle>
            <circle cx="4.5" cy="12.498" r=".6" fill=color></circle>
            <circle cx="6.5" cy="2.5" r=".6" fill=color></circle>
            <circle cx="6.5" cy="4.5" r=".6" fill=color></circle>
            <circle cx="6.5" cy="6.499" r=".6" fill=color></circle>
            <circle cx="6.5" cy="8.499" r=".6" fill=color></circle>
            <circle cx="6.5" cy="10.498" r=".6" fill=color></circle>
            <circle cx="6.5" cy="12.498" r=".6" fill=color></circle>
            <circle cx="8.499" cy="2.5" r=".6" fill=color></circle>
            <circle cx="8.499" cy="4.5" r=".6" fill=color></circle>
            <circle cx="8.499" cy="6.499" r=".6" fill=color></circle>
            <circle cx="8.499" cy="8.499" r=".6" fill=color></circle>
            <circle cx="8.499" cy="10.498" r=".6" fill=color></circle>
            <circle cx="8.499" cy="12.498" r=".6" fill=color></circle>
            <circle cx="10.499" cy="2.5" r=".6" fill=color></circle>
            <circle cx="10.499" cy="4.5" r=".6" fill=color></circle>
            <circle cx="10.499" cy="6.499" r=".6" fill=color></circle>
            <circle cx="10.499" cy="8.499" r=".6" fill=color></circle>
            <circle cx="10.499" cy="10.498" r=".6" fill=color></circle>
            <circle cx="10.499" cy="12.498" r=".6" fill=color></circle>
        </svg>
    }
}
