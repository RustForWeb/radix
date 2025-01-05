use leptos::prelude::*;
use radix_leptos_aspect_ratio::*;

#[component]
pub fn AspectRatioDemo() -> impl IntoView {
    view! {
        <div class="shadow-blackA4 w-[300px] overflow-hidden rounded-md shadow-[0_2px_10px]">
            <AspectRatio ratio=16.0 / 9.0>
                <img
                    class="h-full w-full object-cover"
                    src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
                    alt="Landscape photograph by Tobias Tullius"
                />
            </AspectRatio>
        </div>
    }
}
