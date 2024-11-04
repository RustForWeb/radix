use radix_yew_themes::AspectRatio;
use yew::prelude::*;

#[function_component]
pub fn AspectRatioExample() -> Html {
    html! {
        <AspectRatio ratio={16.0 / 8.0}>
            <img
                src="https://images.unsplash.com/photo-1479030160180-b1860951d696?&auto=format&fit=crop&w=1200&q=80"
                alt="A house in a forest"
                style="object-fit: cover; width: 100%; height: 100%; border-radius: var(--radius-2);"
            />
        </AspectRatio>
    }
}
