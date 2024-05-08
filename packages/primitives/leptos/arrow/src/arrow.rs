use leptos::*;

#[component]
pub fn Arrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    // TODO: support asChild

    let width = move || width().unwrap_or(10.0);
    let height = move || height().unwrap_or(5.0);

    view! {
        <svg
            width=width
            height=height
            class=class
            viewBox="0 0 30 10"
            preserveAspectRatio="none"
            {..attributes}
        >
            <polygon points="0,0 30,0 15,10" />
        </svg>
    }
}
