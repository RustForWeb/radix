use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<View> = vec![];

    if cfg!(feature = "avatar") {
        use crate::avatar::AvatarDemo;
        views.push(view! {
            <AvatarDemo />
        });
    }

    views.into_view()
}
