use leptos::*;
use radix_leptos_popper::*;
use tailwind_fuse::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1 class="text-xl pb-3">Styled</h1>

        <div class="h-[400px] overflow-y-auto">
            <Styled />
        </div>
    }
}

#[component]
fn Styled() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    let anchor_class = create_memo(move |_| AnchorClass::default().to_class());
    let content_class = create_memo(move |_| ContentClass::default().to_class());
    let arrow_class = create_memo(move |_| ArrowClass::default().to_class());

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor class=anchor_class on:click=move |_| set_open(true)>
                    open
                </PopperAnchor>

                <Show when=open>
                    // sideOffset={5}
                    <PopperContent class=content_class>
                        <button on:click=move |_| set_open(false)>close</button>
                        // width={20} height={10}
                        <PopperArrow class=arrow_class />
                    </PopperContent>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
fn Scrollable(children: Children) -> impl IntoView {
    // TODO: figure out how the 200vh works in the stories
    // h-[200vh]
    view! {
        <div class="flex items-center justify-center h-[400px]">
            {children()}
        </div>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "bg-[hotpink]")]
pub struct AnchorClass {
    pub size: AnchorSize,
}

#[derive(TwVariant, Clone, Copy)]
pub enum AnchorSize {
    #[tw(class = "size-[50px]")]
    Small,
    #[tw(default, class = "size-[100px]")]
    Large,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "bg-[#ccc] p-[10px] rounded-[10px]")]
pub struct ContentClass {
    pub size: AnchorSize,
}

#[derive(TwVariant, Clone, Copy)]
pub enum ContentSize {
    #[tw(class = "w-[100px] h-[50px]")]
    Small,
    #[tw(default, class = "w-[300px] h-[100px]")]
    Large,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "fill-[#ccc]")]
pub struct ArrowClass {}
