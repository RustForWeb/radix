// use std::time::Duration;

use radix_yew_popper::*;
// use radix_yew_portal::Portal;
use tailwind_fuse::*;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    let anchor_class = use_memo((), |_| AnchorClass::default().to_class());
    let content_class = use_memo((), |_| ContentClass::default().to_class());
    let arrow_class = use_memo((), |_| ArrowClass::default().to_class());

    let open = use_state_eq(|| false);

    let handle_open = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(true)
    });
    let handle_close = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(false)
    });

    html! {
        <Scrollable>
            <Popper>
                <PopperAnchor class={(*anchor_class).clone()} on_click={handle_open}>
                    {"open"}
                </PopperAnchor>

                if *open {
                    <PopperContent class={(*content_class).clone()} side_offset=5.0>
                        <button onclick={handle_close}>{"close"}</button>
                        <PopperArrow class={(*arrow_class).clone()} width=20.0 height=10.0 />
                    </PopperContent>
                }
            </Popper>
        </Scrollable>
    }
}

#[function_component]
pub fn WithCustomArrow() -> Html {
    let anchor_class = use_memo((), |_| AnchorClass::default().to_class());
    let content_class = use_memo((), |_| ContentClass::default().to_class());

    let open = use_state_eq(|| false);

    let handle_open = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(true)
    });
    let handle_close = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(false)
    });

    html! {
        <Scrollable>
            <Popper>
                <PopperAnchor class={(*anchor_class).clone()} on_click={handle_open}>
                    {"open"}
                </PopperAnchor>

                if *open {
                    <PopperContent class={(*content_class).clone()} side={Side::Right} side_offset=5.0>
                        <button onclick={handle_close}>{"close"}</button>
                        <PopperArrow
                            as_child={Callback::from(|PopperArrowChildProps { node_ref, style, .. }| html! {
                                <CustomArrow node_ref={node_ref} style={style} />
                            })}
                        />
                    </PopperContent>
                }
            </Popper>
        </Scrollable>
    }
}

#[function_component]
pub fn Animated() -> Html {
    let anchor_class = use_memo((), |_| AnchorClass::default().to_class());
    let animated_content_class = use_memo((), |_| AnimatedContentClass::default().to_class());
    let arrow_class = use_memo((), |_| ArrowClass::default().to_class());

    let open = use_state_eq(|| false);

    let handle_open = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(true)
    });
    let handle_close = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(false)
    });

    html! {
        <Scrollable>
            <Popper>
                <PopperAnchor class={(*anchor_class).clone()} on_click={handle_open}>
                    {"open"}
                </PopperAnchor>

                if *open {
                    <PopperContent class={(*animated_content_class).clone()} side_offset=5.0>
                        <button onclick={handle_close}>{"close"}</button>
                        <PopperArrow class={(*arrow_class).clone()} width=20.0 height=10.0 />
                    </PopperContent>
                }
            </Popper>
        </Scrollable>
    }
}

// TODO: enable Portal
#[function_component]
pub fn WithPortal() -> Html {
    let anchor_class = use_memo((), |_| AnchorClass::default().to_class());
    let content_class = use_memo((), |_| ContentClass::default().to_class());
    let arrow_class = use_memo((), |_| ArrowClass::default().to_class());

    let open = use_state_eq(|| false);

    let handle_open = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(true)
    });
    let handle_close = use_callback((), {
        let open = open.clone();

        move |_, _| open.set(false)
    });

    html! {
        <Scrollable>
            <Popper>
                <PopperAnchor class={(*anchor_class).clone()} on_click={handle_open}>
                    {"open"}
                </PopperAnchor>

                if *open {
                    // <Portal as_child=true>
                        <PopperContent class={(*content_class).clone()} side_offset=5.0>
                            <button onclick={handle_close}>{"close"}</button>
                            <PopperArrow class={(*arrow_class).clone()} width=20.0 height=10.0 />
                        </PopperContent>
                    // </Portal>
                }
            </Popper>
        </Scrollable>
    }
}

#[function_component]
pub fn WithUpdatePositionStrategyAlways() -> Html {
    //     let anchor_class = use_memo((), |_| AnchorClass::default().to_class());
    //     let content_class = use_memo((), |_| ContentClass::default().to_class());
    //     let arrow_class = use_memo((), |_| ArrowClass::default().to_class());

    //     let open = use_state_eq(|| false);
    //     let left = use_state_eq(|| 0);

    //     let handle = set_interval_with_handle(
    //         move || {
    //             set_left.update(move |left| {
    //                 *left = (*left + 50) % 300;
    //             });
    //         },
    //         Duration::from_millis(500),
    //     )
    //     .expect("Interval should be started");

    //     on_cleanup(move || {
    //         handle.clear();
    //     });

    //     html! {
    //         <Scrollable>
    //             <Popper>
    //                 <PopperAnchor attr:class=anchor_class on:click=move |_| set_open.set(true) attr:style=move || format!("margin-left: {}px;", left.get())>
    //                     open
    //                 </PopperAnchor>

    //                 <Show when=move || open.get()>
    //                     <Portal as_child=true>
    //                         <PopperContent attr:class=content_class side_offset=5.0 update_position_strategy=UpdatePositionStrategy::Always>
    //                             <button on:click=move |_| set_open.set(false)>close</button>
    //                             <PopperArrow attr:class=arrow_class width=20.0 height=10.0 />
    //                         </PopperContent>
    //                     </Portal>
    //                 </Show>
    //             </Popper>
    //         </Scrollable>
    //     }

    html! {}
}

#[function_component]
pub fn Chromatic() -> Html {
    //     let anchor_class = use_memo((), |_| {
    //         AnchorClass::builder().size(AnchorSize::Small).to_class()
    //     });
    //     let content_class = use_memo((), |_| {
    //         ContentClass::builder().size(ContentSize::Small).to_class()
    //     });
    //     let arrow_class = use_memo((), |_| ArrowClass::default().to_class());

    //     let scroll_container1_ref = use_node_ref();
    //     let scroll_container1: Signal<Vec<web_sys::Element>> = Signal::derive(move || {
    //         scroll_container1_ref
    //             .get()
    //             .map(|scroll_container| {
    //                 let element: &web_sys::HtmlDivElement = &scroll_container;
    //                 vec![element.clone().into()]
    //             })
    //             .unwrap_or(vec![])
    //     });
    //     let scroll_container2_ref = use_node_ref();
    //     let scroll_container2: Signal<Vec<web_sys::Element>> = Signal::derive(move || {
    //         scroll_container2_ref
    //             .get()
    //             .map(|scroll_container| {
    //                 let element: &web_sys::HtmlDivElement = &scroll_container;
    //                 vec![element.clone().into()]
    //             })
    //             .unwrap_or(vec![])
    //     });

    //     html! {
    //         <div style="padding-bottom: 500px;">
    //             <header style="display: flex; align-items: center; justify-content: center; gap: 150px; position: fixed; top: 0px; left: 0px; right: 0px; background-color: grey; border: 1px solid black;">
    //                 <h1>{"In fixed header"}</h1>
    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>1</PopperAnchor>
    //                     <PopperContent attr:class=content_class side_offset=5.0>
    //                         <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"1"}
    //                     </PopperContent>
    //                 </Popper>

    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>2</PopperAnchor>
    //                     <Portal as_child=true>
    //                         <PopperContent attr:class=content_class side_offset=5.0>
    //                             <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"2 (portalled)"}
    //                         </PopperContent>
    //                     </Portal>
    //                 </Popper>
    //             </header>

    //             <div
    //                 style:margin-top="100px"
    //                 style:display="flex"
    //                 style:align-items="center"
    //                 style:justify-content="center"
    //                 style:gap="150px"
    //                 style:border="1px solid black"
    //             >
    //                 <h1>{"In normal page flow"}</h1>
    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>3</PopperAnchor>
    //                     <PopperContent attr:class=content_class side_offset=5.0>
    //                         <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"3"}
    //                     </PopperContent>
    //                 </Popper>

    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>4</PopperAnchor>
    //                     <Portal as_child=true>
    //                         <PopperContent attr:class=content_class side_offset=5.0>
    //                             <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"4 (portalled)"}
    //                         </PopperContent>
    //                     </Portal>
    //                 </Popper>
    //             </div>

    //             <div
    //                 style:position="relative"
    //                 style:margin-top="50px"
    //                 style:display="flex"
    //                 style:align-items="center"
    //                 style:justify-content="center"
    //                 style:gap="150px"
    //                 style:border="1px solid black"
    //             >
    //                 <h1>{"In relative parent"}</h1>
    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>5</PopperAnchor>
    //                     <PopperContent attr:class=content_class side_offset=5.0>
    //                         <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"5"}
    //                     </PopperContent>
    //                 </Popper>

    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>6</PopperAnchor>
    //                     <Portal as_child=true>
    //                         <PopperContent attr:class=content_class side_offset=5.0>
    //                             <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"6 (portalled)"}
    //                         </PopperContent>
    //                     </Portal>
    //                 </Popper>
    //             </div>

    //             <div
    //                 style:margin-top="50px"
    //                 style:display="flex"
    //                 style:align-items="center"
    //                 style:justify-content="center"
    //                 style:gap="150px"
    //                 style:border="1px solid black"
    //                 style:transform="translate3d(100px, 0, 0)"
    //             >
    //                 <h1>{"In translated parent"}</h1>
    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>7</PopperAnchor>
    //                     <PopperContent attr:class=content_class side_offset=5.0>
    //                         <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"7"}
    //                     </PopperContent>
    //                 </Popper>

    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>8</PopperAnchor>
    //                     <Portal as_child=true>
    //                         <PopperContent attr:class=content_class side_offset=5.0>
    //                             <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />{"8 (portalled)"}
    //                         </PopperContent>
    //                     </Portal>
    //                 </Popper>
    //             </div>

    //             <div style:display="flex" style:gap="100px">
    //                 <div>
    //                     <h1>{"In scrolling container"}</h1>
    //                     <div
    //                         node_ref=scroll_container1_ref
    //                         style:width="400px"
    //                         style:height="600px"
    //                         style:overflow="auto"
    //                         style:border="1px solid black"
    //                     >
    //                         <div style:height="2000px">
    //                             <For
    //                                 each=|| 1..11
    //                                 key=|i| *i
    //                                 children=move |i| html! {
    //                                     <div
    //                                         style:display="flex"
    //                                         style:align-items="center"
    //                                         style:justify-content="center"
    //                                         style:gap="150px"
    //                                         style:padding-bottom="100px"
    //                                     >
    //                                         <Popper>
    //                                             <PopperAnchor attr:class=anchor_class>{"9."}{i + 1}</PopperAnchor>
    //                                             <PopperContent
    //                                                 attr:class=content_class
    //                                                 side_offset=5.0
    //                                                 hide_when_detached=true
    //                                                 collision_boundary=scroll_container1
    //                                             >
    //                                                 <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                                                 {"9."}{i + 1}
    //                                             </PopperContent>
    //                                         </Popper>

    //                                         <Popper>
    //                                             <PopperAnchor attr:class=anchor_class>10.{i + 1}</PopperAnchor>
    //                                             <Portal as_child=true>
    //                                                 <PopperContent
    //                                                     attr:class=content_class
    //                                                     side_offset=5.0
    //                                                     hide_when_detached=true
    //                                                     collision_boundary=scroll_container1
    //                                                 >
    //                                                     <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                                                     {"10."}{i + 1}
    //                                                 </PopperContent>
    //                                             </Portal>
    //                                         </Popper>
    //                                     </div>
    //                                 }
    //                             />
    //                         </div>
    //                     </div>
    //                 </div>

    //                 <div>
    //                     <h1>{"With position sticky"}</h1>
    //                     <div
    //                         node_ref=scroll_container2_ref
    //                         style:width="400px"
    //                         style:height="600px"
    //                         style:overflow="auto"
    //                         style:border="1px solid black"
    //                     >
    //                         <div style:height="2000px">
    //                             <For
    //                                 each=|| 1..11
    //                                 key=|i| *i
    //                                 children=move |i| html! {
    //                                     <div
    //                                         style:display="flex"
    //                                         style:align-items="center"
    //                                         style:justify-content="center"
    //                                         style:gap="150px"
    //                                         style:padding-bottom="100px"
    //                                         style:position="sticky"
    //                                         style:top="0px"
    //                                     >
    //                                         <Popper>
    //                                             <PopperAnchor attr:class=anchor_class>9.{i + 1}</PopperAnchor>
    //                                             <PopperContent
    //                                                 attr:class=content_class
    //                                                 side_offset=5.0
    //                                                 hide_when_detached=true
    //                                                 collision_boundary=scroll_container2
    //                                             >
    //                                                 <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                                                 {"9."}{i + 1}
    //                                             </PopperContent>
    //                                         </Popper>

    //                                         <Popper>
    //                                             <PopperAnchor attr:class=anchor_class>10.{i + 1}</PopperAnchor>
    //                                             <Portal as_child=true>
    //                                                 <PopperContent
    //                                                     attr:class=content_class
    //                                                     side_offset=5.0
    //                                                     hide_when_detached=true
    //                                                     collision_boundary=scroll_container2
    //                                                 >
    //                                                     <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                                                     {"10."}{i + 1}
    //                                                 </PopperContent>
    //                                             </Portal>
    //                                         </Popper>
    //                                     </div>
    //                                 }
    //                             />
    //                         </div>
    //                     </div>
    //                 </div>
    //             </div>

    //             <div
    //                 style:margin-top="50px"
    //                 style:display="flex"
    //                 style:align-items="center"
    //                 style:justify-content="center"
    //                 style:gap="150px"
    //                 style:border="1px solid black"
    //             >
    //                 <h1>{"Logical \"start\" alignment (LTR)"}</h1>
    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>11</PopperAnchor>
    //                     <PopperContent attr:class=content_class align=Align::Start side_offset=5.0>
    //                         <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                         {"11"}
    //                     </PopperContent>
    //                 </Popper>

    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>12</PopperAnchor>
    //                     <Portal as_child=true>
    //                         <PopperContent attr:class=content_class align=Align::Start side_offset=5.0>
    //                             <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                             {"12 (portalled)"}
    //                         </PopperContent>
    //                     </Portal>
    //                 </Popper>
    //             </div>

    //             <div
    //                 style:margin-top="50px"
    //                 style:display="flex"
    //                 style:align-items="center"
    //                 style:justify-content="center"
    //                 style:gap="150px"
    //                 style:border="1px solid black"
    //             >
    //                 <h1>{"Logical \"start\" alignment (RTL)"}</h1>
    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>13</PopperAnchor>
    //                     <PopperContent attr:class=content_class attr:dir="rtl" align=Align::Start side_offset=5.0>
    //                         <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                         {"13"}
    //                     </PopperContent>
    //                 </Popper>

    //                 <Popper>
    //                     <PopperAnchor attr:class=anchor_class>14</PopperAnchor>
    //                     <Portal as_child=true>
    //                         <PopperContent attr:class=content_class attr:dir="rtl" align=Align::Start side_offset=5.0>
    //                             <PopperArrow attr:class=arrow_class width=10.0 height=5.0 />
    //                             {"14 (portalled)"}
    //                         </PopperContent>
    //                     </Portal>
    //                 </Popper>
    //             </div>
    //         </div>
    //     }

    html! {}
}

#[derive(PartialEq, Properties)]
struct ScrollableProps {
    #[prop_or_default]
    children: Html,
}

#[function_component]
fn Scrollable(props: &ScrollableProps) -> Html {
    html! {
        <div class="flex items-center justify-center h-[200vh]">
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct CustomArrowProps {
    #[prop_or_default]
    style: Option<String>,
    #[prop_or_default]
    node_ref: NodeRef,
}

#[function_component]
fn CustomArrow(props: &CustomArrowProps) -> Html {
    html! {
        <div
            ref={props.node_ref.clone()}
            style={format!("width: 20px; height: 20px; border-bottom-left-radius: 10px; border-bottom-right-radius: 10px; background-color: tomato;{}", props.style.clone().unwrap_or_default())}
        />
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "origin-[--radix-popper-transform-origin] bg-[#ccc] p-[10px] rounded-[10px]")]
struct ContentClass {
    pub size: ContentSize,
}

#[derive(TwVariant)]
enum ContentSize {
    #[tw(class = "w-[100px] h-[50px]")]
    #[allow(dead_code)]
    Small,
    #[tw(default, class = "w-[300px] h-[150px]")]
    Large,
}

#[derive(TwVariant)]
enum AnchorSize {
    #[tw(class = "size-[50px]")]
    #[allow(dead_code)]
    Small,
    #[tw(default, class = "size-[100px]")]
    Large,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "bg-[hotpink]")]
struct AnchorClass {
    pub size: AnchorSize,
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "fill-[#ccc]")]
struct ArrowClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[#ccc] p-[10px] rounded-[10px] data-[side=top]:[--direction:1] data-[side=bottom]:[--direction:-1] animate-[popperRotateIn_0.6s_cubic-bezier(0.16,1,0.3,1)]"
)]
struct AnimatedContentClass {
    pub size: ContentSize,
}
