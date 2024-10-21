use radix_yew_separator::*;
use yew::prelude::*;

#[function_component]
pub fn SeparatorDemo() -> Html {
    html! {
        <div class="w-full max-w-[300px] mx-[15px]">
            <div class="text-white text-[15px] leading-5 font-medium">{"Radix Primitives"}</div>
            <div class="text-white text-[15px] leading-5">{"An open-source UI component library."}</div>
            <Separator class="bg-violet6 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px my-[15px]" />
            <div class="flex h-5 items-center">
                <div class="text-white text-[15px] leading-5">{"Blog"}</div>
                <Separator
                    class="bg-violet6 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px mx-[15px]"
                    decorative=true
                    orientation={Orientation::Vertical}
                />
                <div class="text-white text-[15px] leading-5">{"Docs"}</div>
                <Separator
                    class="bg-violet6 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px mx-[15px]"
                    decorative=true
                    orientation={Orientation::Vertical}
                />
                <div class="text-white text-[15px] leading-5">{"Source"}</div>
            </div>
        </div>
    }
}
