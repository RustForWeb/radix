use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let mut views: Vec<View> = vec![];

    #[cfg(feature = "aspect-ratio")]
    {
        use crate::aspect_ratio::AspectRatioDemo;
        views.push(view! {
            <AspectRatioDemo />
        });
    }
    #[cfg(feature = "avatar")]
    {
        use crate::avatar::AvatarDemo;
        views.push(view! {
            <AvatarDemo />
        });
    }
    #[cfg(feature = "label")]
    {
        use crate::label::LabelDemo;
        views.push(view! {
            <LabelDemo />
        });
    }
    #[cfg(feature = "progress")]
    {
        use crate::progress::ProgressDemo;
        views.push(view! {
            <ProgressDemo />
        });
    }
    #[cfg(feature = "separator")]
    {
        use crate::separator::SeparatorDemo;
        views.push(view! {
            <SeparatorDemo />
        });
    }
    #[cfg(feature = "toggle")]
    {
        use crate::toggle::ToggleDemo;
        views.push(view! {
            <ToggleDemo />
        });
    }

    view! {
        <div class="w-full h-full flex justify-center items-start">
            {views.into_view()}
        </div>
    }
}
