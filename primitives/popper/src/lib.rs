use leptos::*;

#[component]
pub fn Popper(children: Children) -> impl IntoView {
    // TODO: provide context

    view! {
        {children()}
    }
}

#[component]
pub fn PopperAnchor(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || class.get() {..attributes}>
            {children()}
        </div>
    }
}

#[component]
pub fn PopperContent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {
        // TODO: add Floating UI
        <div
            style:min-width="max-content"
        >
            <div class=move || class.get() {..attributes}>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn PopperArrow(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {}
}
