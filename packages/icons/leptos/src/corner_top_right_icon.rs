use leptos::{prelude::*, svg::Svg};
#[component]
pub fn CornerTopRightIcon(
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
                d="M5.12263 3H5.1H3.5C3.22386 3 3 3.22386 3 3.5C3 3.77614 3.22386 4 3.5 4H5.1C6.22836 4 7.04455 4.00039 7.68648 4.05284C8.32256 4.10481 8.74338 4.20539 9.08897 4.38148C9.74753 4.71703 10.283 5.25247 10.6185 5.91103C10.7946 6.25662 10.8952 6.67744 10.9472 7.31352C10.9996 7.95545 11 8.77164 11 9.9V11.5C11 11.7761 11.2239 12 11.5 12C11.7761 12 12 11.7761 12 11.5V9.9V9.87737C12 8.77641 12 7.91948 11.9438 7.23209C11.8868 6.53416 11.7694 5.9671 11.5095 5.45704C11.0781 4.61031 10.3897 3.9219 9.54296 3.49047C9.0329 3.23058 8.46584 3.11318 7.76791 3.05616C7.08052 3 6.22359 3 5.12263 3Z"
                fill=color
            ></path>
        </svg>
    }
}
