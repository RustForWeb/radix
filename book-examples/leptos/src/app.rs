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

    view! {
        <div class="w-full h-full flex justify-center items-center">
            {views.into_view()}
        </div>
    }
}
