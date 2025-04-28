use radix_yew_avatar::*;
use tailwind_fuse::*;
use yew::prelude::*;

const SRC: &str = "https://picsum.photos/id/1005/400/400";
const SRC_BROKEN: &str = "https://broken.link.com/broken-pic.jpg";

#[function_component]
pub fn Styled() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let image_class = use_memo((), |_| ImageClass::default().to_class());
    let fallback_class = use_memo((), |_| FallbackClass::default().to_class());

    html! {
        <>
            <h1>{"Without image & with fallback"}</h1>
            <Avatar class={(*root_class).clone()}>
                <AvatarFallback class={(*fallback_class).clone()}>{"JS"}</AvatarFallback>
            </Avatar>

            <h1>{"With image & with fallback"}</h1>
            <Avatar class={(*root_class).clone()}>
                <AvatarImage
                    class={(*image_class).clone()}
                    alt="John Smith"
                    src={SRC}
                />
                <AvatarFallback class={(*fallback_class).clone()} delay_ms=300>
                    {"JS"}
                </AvatarFallback>
            </Avatar>

            <h1>{"With image & with fallback (but broken src)"}</h1>
            <Avatar class={(*root_class).clone()}>
                <AvatarImage
                    class={(*image_class).clone()}
                    alt="John Smith"
                    src={SRC_BROKEN}
                    on_loading_status_change={move |status| log::info!("{status:?}")}
                />
                <AvatarFallback class={(*fallback_class).clone()}>
                    <AvatarIcon />
                </AvatarFallback>
            </Avatar>
        </>
    }
}

#[function_component]
pub fn Chromatic() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let image_class = use_memo((), |_| ImageClass::default().to_class());
    let fallback_class = use_memo((), |_| FallbackClass::default().to_class());

    html! {
        <>
            <h1>{"Without image & with fallback"}</h1>
            <Avatar class={(*root_class).clone()}>
                <AvatarFallback class={(*fallback_class).clone()}>{"JS"}</AvatarFallback>
            </Avatar>

            <h1>{"With image & with fallback"}</h1>
            <Avatar class={(*root_class).clone()}>
                <AvatarImage class={(*image_class).clone()} alt="John Smith" src={SRC} />
                <AvatarFallback class={(*fallback_class).clone()} delay_ms=300>
                    {"JS"}
                </AvatarFallback>
            </Avatar>

            <h1>{"With image & with fallback (but broken src)"}</h1>
            <Avatar class={(*root_class).clone()}>
                <AvatarImage class={(*image_class).clone()} alt="John Smith" src={SRC_BROKEN} />
                <AvatarFallback class={(*fallback_class).clone()}>
                    <AvatarIcon />
                </AvatarFallback>
            </Avatar>
        </>
    }
}

#[function_component]
fn AvatarIcon() -> Html {
    html! {
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
struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "w-full h-full object-cover")]
struct ImageClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "w-full h-full flex items-center justify-center bg-[#111] text-[#fff]")]
struct FallbackClass {}
