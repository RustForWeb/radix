use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<View> = vec![];

    #[cfg(feature = "avatar")]
    {
        use crate::avatar::AvatarDemo;
        views.push(view! {
            <AvatarDemo />
        });
    }
    #[cfg(feature = "separator")]
    {
        use crate::separator::SeparatorDemo;
        views.push(view! {
            <SeparatorDemo />
        });
    }

    view! {
        <div class="w-full h-full flex justify-center items-start">
            {views.into_view()}
        </div>
    }
}
