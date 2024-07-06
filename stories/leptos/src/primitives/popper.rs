use leptos::*;
use radix_leptos_popper::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    let anchor_class = Memo::new(move |_| AnchorClass::default().to_class());
    let content_class = Memo::new(move |_| ContentClass::default().to_class());
    let arrow_class = Memo::new(move |_| ArrowClass::default().to_class());

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor attr:class=anchor_class on:click=move |_| set_open.set(true)>
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <PopperContent attr:class=content_class side_offset=5.0>
                        <button on:click=move |_| set_open.set(false)>close</button>
                        <PopperArrow attr:class=arrow_class width=20.0 height=10.0 />
                    </PopperContent>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn WithCustomArrow() -> impl IntoView {
    view! {}
}

#[component]
pub fn Animated() -> impl IntoView {
    view! {}
}

#[component]
pub fn WithPortal() -> impl IntoView {
    view! {}
}

#[component]
pub fn WithUpdatePositionStrategyAlways() -> impl IntoView {
    view! {}
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {}
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

#[derive(TwVariant)]
pub enum AnchorSize {
    #[tw(class = "size-[50px]")]
    #[allow(dead_code)]
    Small,
    #[tw(default, class = "size-[100px]")]
    Large,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "bg-[#ccc] p-[10px] rounded-[10px]")]
pub struct ContentClass {
    pub size: ContentSize,
}

#[derive(TwVariant)]
pub enum ContentSize {
    #[tw(class = "w-[100px] h-[50px]")]
    #[allow(dead_code)]
    Small,
    #[tw(default, class = "w-[300px] h-[150px]")]
    Large,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "fill-[#ccc]")]
pub struct ArrowClass {}
