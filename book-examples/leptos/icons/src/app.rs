use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    #[allow(unused_mut)]
    let mut views: Vec<View> = vec![];

    #[cfg(feature = "icons")]
    {
        use crate::icons::IconsDemo;
        views.push(view! {
            <IconsDemo />
        });
    }

    view! {
        <div class="w-full h-full flex justify-center items-start">
            {views.into_view()}
        </div>
    }
}
