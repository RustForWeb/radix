use radix_yew_aspect_ratio::*;
use tailwind_fuse::*;
use yew::prelude::*;

#[function_component]
fn Image() -> Html {
    html! {
        <img
            src="https://images.unsplash.com/photo-1605030753481-bb38b08c384a?&auto=format&fit=crop&w=400&q=80"
            alt="A house in a forest"
            style="object-fit: cover; width: 100%; height: 100%;"
        />
    }
}

#[function_component]
pub fn Styled() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());

    html! {
        <div style="width: 500px;">
            <AspectRatio class={(*root_class).clone()}>
                <h1>{"Default ratio (1/1)"}</h1>
            </AspectRatio>
        </div>
    }
}

#[function_component]
pub fn CustomRatios() -> Html {
    html! {
        <div style="display: flex; gap: 20px;">
            <div style="width: 200px;">
                <AspectRatio ratio={1.0 / 2.0}><Image /></AspectRatio>
            </div>
            <div style="width: 200px;">
                <AspectRatio><Image /></AspectRatio>
            </div>
            <div style="width: 200px;">
                <AspectRatio ratio={16.0 / 9.0}><Image /></AspectRatio>
            </div>
            <div style="width: 200px;">
                <AspectRatio ratio={2.0 / 1.0}><Image /></AspectRatio>
            </div>
        </div>
    }
}

#[function_component]
pub fn Chromatic() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());

    html! {
        <>
            <h1>{"Default ratio"}</h1>
            <div style="width: 300px;">
                <AspectRatio class={(*root_class).clone()}>
                    <h1>{"Default ratio (1/1)"}</h1>
                </AspectRatio>
            </div>

            <h1>{"Custom ratios"}</h1>
            <div style="display: flex; gap: 20px;">
                <div style="width: 200px;">
                    <AspectRatio ratio={1.0 / 2.0}><Image /></AspectRatio>
                </div>
                <div style="width: 200px;">
                    <AspectRatio><Image /></AspectRatio>
                </div>
                <div style="width: 200px;">
                    <AspectRatio ratio={16.0 / 9.0}><Image /></AspectRatio>
                </div>
                <div style="width: 200px;">
                    <AspectRatio ratio={2.0 / 1.0}><Image /></AspectRatio>
                </div>
            </div>
        </>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "flex items-center justify-center bg-[crimson] text-[#fff]")]
struct RootClass {}
