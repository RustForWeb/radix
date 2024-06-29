use leptos::*;
use radix_leptos_avatar::*;
use tailwind_fuse::*;

const SRC: &str = "https://picsum.photos/id/1005/400/400";
const SRC_BROKEN: &str = "https://broken.link.com/broken-pic.jpg";

#[component]
pub fn Styled() -> impl IntoView {
    let root_class = create_memo(move |_| RootClass::default().to_class());
    let image_class = create_memo(move |_| ImageClass::default().to_class());
    let fallback_class = create_memo(move |_| FallbackClass::default().to_class());

    view! {
        <h1>Without image & with fallback</h1>
        <Avatar attr:class=root_class>
            <AvatarFallback attr:class=fallback_class>JS</AvatarFallback>
        </Avatar>

        <h1>With image & with fallback</h1>
        <Avatar attr:class=root_class>
            <AvatarImage
                src=SRC
                attr:alt="John Smith"
                attr:class=image_class
            />
            <AvatarFallback delay_ms=300 attr:class=fallback_class>
                JS
            </AvatarFallback>
        </Avatar>

        <h1>With image & with fallback (but broken src)</h1>
        <Avatar attr:class=root_class>
            <AvatarImage
                src=SRC_BROKEN
                on_loading_status_change=move |status| log::info!("{:?}", status)
                attr:alt="John Smith"
                attr:class=image_class
            />
            <AvatarFallback attr:class=fallback_class>
                <AvatarIcon />
            </AvatarFallback>
        </Avatar>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {}
}

#[component]
fn AvatarIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="42" height="42">
            <path
                d="M50 51.7a22.1 22.1 0 100-44.2 22.1 22.1 0 000 44.2zM87.9 69.3a27.8 27.8 0 00-21.2-16.1 4 4 0 00-2.8.7 23.5 23.5 0 01-27.6 0 4 4 0 00-2.8-.7 27.5 27.5 0 00-21.2 16.1c-.3.6-.2 1.3.1 1.8a52.8 52.8 0 007 8.9 43.4 43.4 0 0056.9 3.8 56.3 56.3 0 008.9-8.8c.9-1.2 1.8-2.5 2.6-3.9.3-.6.3-1.2.1-1.8z"
                fill="currentColor"
            />
        </svg>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-flex items-center justify-center align-middle overflow-hidden select-none rounded-[9999px] w-[48px] h-[48px]"
)]
pub struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "w-full h-full object-cover")]
pub struct ImageClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "w-full h-full flex items-center justify-center bg-[#111] text-[#fff]")]
pub struct FallbackClass {}
