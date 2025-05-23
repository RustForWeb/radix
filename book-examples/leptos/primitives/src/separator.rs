use leptos::prelude::*;
use radix_leptos_separator::*;

#[component]
pub fn SeparatorDemo() -> impl IntoView {
    view! {
        <div class="w-full max-w-[300px] mx-[15px]">
            <div class="text-white text-[15px] leading-5 font-medium">Radix Primitives</div>
            <div class="text-white text-[15px] leading-5">An open-source UI component library.</div>
            <Separator
                attr:class="bg-violet6 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px my-[15px]"
            />
            <div class="flex h-5 items-center">
                <div class="text-white text-[15px] leading-5">Blog</div>
                <Separator
                    attr:class="bg-violet6 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px mx-[15px]"
                    decorative=true
                    orientation=Orientation::Vertical
                />
                <div class="text-white text-[15px] leading-5">Docs</div>
                <Separator
                    attr:class="bg-violet6 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px mx-[15px]"
                    decorative=true
                    orientation=Orientation::Vertical
                />
                <div class="text-white text-[15px] leading-5">Source</div>
            </div>
        </div>
    }
}
