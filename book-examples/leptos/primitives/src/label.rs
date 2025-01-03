use leptos::prelude::*;
use radix_leptos_label::*;

#[component]
pub fn LabelDemo() -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-[15px] px-5">
            <Label attr:class="text-[15px] font-medium leading-[35px] text-white" attr:r#for="firstName">
                First name
            </Label>
            <input
                class="bg-blackA2 shadow-blackA6 inline-flex h-[35px] w-[200px] appearance-none items-center justify-center rounded-[4px] px-[10px] text-[15px] leading-none text-white shadow-[0_0_0_1px] outline-none focus:shadow-[0_0_0_2px_black] selection:color-white selection:bg-blackA6"
                type="text"
                id="firstName"
                value="Pedro Duarte"
            />
        </div>
    }
}
