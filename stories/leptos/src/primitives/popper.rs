use leptos::*;
use radix_leptos_popper::*;
use radix_leptos_portal::Portal;
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
    let (open, set_open) = create_signal(false);

    let anchor_class = Memo::new(move |_| AnchorClass::default().to_class());
    let content_class = Memo::new(move |_| ContentClass::default().to_class());

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor attr:class=anchor_class on:click=move |_| set_open.set(true)>
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <PopperContent attr:class=content_class side=Side::Right side_offset=5.0>
                        <button on:click=move |_| set_open.set(false)>close</button>
                        <PopperArrow as_child=true attr:offset=20>
                            <CustomArrow />
                        </PopperArrow>
                    </PopperContent>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    let anchor_class = Memo::new(move |_| AnchorClass::default().to_class());
    let animated_content_class = Memo::new(move |_| AnimatedContentClass::default().to_class());
    let arrow_class = Memo::new(move |_| ArrowClass::default().to_class());

    view! {
        <Scrollable>
            <Popper>
                <PopperAnchor attr:class=anchor_class on:click=move |_| set_open.set(true)>
                    open
                </PopperAnchor>

                <Show when=move || open.get()>
                    <PopperContent attr:class=animated_content_class side_offset=5.0>
                        <button on:click=move |_| set_open.set(false)>close</button>
                        <PopperArrow attr:class=arrow_class width=20.0 height=10.0 attr:offset=25 />
                    </PopperContent>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn WithPortal() -> impl IntoView {
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
                    <Portal as_child=true>
                        <PopperContent attr:class=content_class side_offset=5.0>
                            <button on:click=move |_| set_open.set(false)>close</button>
                            <PopperArrow attr:class=arrow_class width=20.0 height=10.0 />
                        </PopperContent>
                    </Portal>
                </Show>
            </Popper>
        </Scrollable>
    }
}

#[component]
pub fn WithUpdatePositionStrategyAlways() -> impl IntoView {
    // TODO
    view! {}
}

#[component]
pub fn Chromatic() -> impl IntoView {
    // TODO
    view! {}
}

#[component]
fn Scrollable(children: Children) -> impl IntoView {
    view! {
        <div class="flex items-center justify-center h-[200vh]">
            {children()}
        </div>
    }
}

#[component]
fn CustomArrow(#[prop(attrs)] attrs: Vec<(&'static str, Attribute)>) -> impl IntoView {
    view! {
        <div
            {..attrs}
            style:width="20px"
            style:height="10px"
            style:border-bottom-left-radius="10px"
            style:border-bottom-right-radius="10px"
            style:background-color="tomato"
        />
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "origin-[--radix-popper-transform-origin] bg-[#ccc] p-[10px] rounded-[10px]")]
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

#[derive(TwVariant)]
pub enum AnchorSize {
    #[tw(class = "size-[50px]")]
    #[allow(dead_code)]
    Small,
    #[tw(default, class = "size-[100px]")]
    Large,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "bg-[hotpink]")]
pub struct AnchorClass {
    pub size: AnchorSize,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "fill-[#ccc]")]
pub struct ArrowClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[#ccc] p-[10px] rounded-[10px] data-[side=top]:[--direction:1] data-[side=bottom]:[--direction:-1] animate-[popperRotateIn_0.6s_cubic-bezier(0.16,1,0.3,1)]"
)]
pub struct AnimatedContentClass {
    pub size: ContentSize,
}
