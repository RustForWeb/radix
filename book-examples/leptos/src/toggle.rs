use leptos::*;
use radix_leptos_toggle::*;

#[component]
pub fn ToggleDemo() -> impl IntoView {
    view! {
        <Toggle
            attr:aria-label="Toggle italic"
            attr:class="hover:bg-violet3 color-mauve11 data-[state=on]:bg-violet6 data-[state=on]:text-violet12 shadow-blackA4 flex h-[35px] w-[35px] items-center justify-center rounded bg-white text-base leading-4 shadow-[0_2px_10px] focus:shadow-[0_0_0_2px] focus:shadow-black"
        >
            {/* <FontItalicIcon /> */}
            I
        </Toggle>
    }
}
