use radix_yew_portal::Portal;
use yew::prelude::*;

#[function_component]
pub fn Base() -> Html {
    html! {
        <div style="max-width: 300px; max-height: 200px; overflow: auto; border: 1px solid;">
            <h1>{"This content is rendered in the main DOM tree"}</h1>
            <p>
                {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos porro, est ex quia itaque facere
                fugit necessitatibus aut enim. Nisi rerum quae, repellat in perspiciatis explicabo laboriosam
                necessitatibus eius pariatur."}
            </p>

            <Portal>
                <h1>{"This content is rendered in a portal (another DOM tree)"}</h1>
                <p>
                    {"Because of the portal, it can appear in a different DOM tree from the main one (by default a
                    new element inside the body), even though it is part of the same Yew tree."}
                </p>
            </Portal>
        </div>
    }
}

#[function_component]
pub fn CustomContainer() -> Html {
    let portal_container_ref = use_node_ref();

    html! {
        <>
            <div style="max-width: 300px; padding: 10px; margin: 10px; border: 1px solid;">
                <h1>{"Container A"}</h1>
                <Portal
                    container_ref={portal_container_ref.clone()}

                    as_child={Callback::from(|_| html! {
                        <p>
                            {"This content is rendered in a portal inside Container A but appears inside Container B
                            because we have used Container B as a container element for the Portal."}
                        </p>
                    })}
                />
            </div>

            <div ref={portal_container_ref} style="max-width: 300px; padding: 10px; margin: 10px; border: 1px solid;">
                <h1>{"Container B"}</h1>
            </div>
        </>
    }
}

#[function_component]
pub fn Chromatic() -> Html {
    let portal_container_ref = use_node_ref();

    html! {
        <div style="padding: 150px;">
            <h1>{"Default (append to body)"}</h1>
            <div style="padding: 10px; margin: 10px; border: 1px solid blue;">
                <p>{"Container A"}</p>

                <Portal
                    as_child={Callback::from(|_| html! {
                        <div style="padding: 10px; margin: 10px; border: 1px solid blue; position: absolute; top: 0px; left: 0px; z-index: 9999999;">
                            <p>{"This content is rendered in a portal (another DOM tree)"}</p>
                            <p>
                                {"Because of the portal, it can appear in a different DOM tree from the main one (by
                                default a new element inside the body), even though it is part of the same Yew tree."}
                            </p>
                        </div>
                    })}
                />
            </div>

            <h1>{"Custom container"}</h1>
            <div style="padding: 10px; margin: 10px; border: 1px solid green;">
                <h1>{"Container B"}</h1>
                <Portal
                    container_ref={portal_container_ref.clone()}

                    as_child={Callback::from(|_| html! {
                        <div style="padding: 10px; margin: 10px; border: 1px solid green;">
                            <p>
                                {"This content is rendered in a portal inside Container B but appears inside Container C
                                because we have used Container C as a container element for the Portal."}
                            </p>
                        </div>
                    })}
                />
            </div>

            <div ref={portal_container_ref} style="padding: 10px; margin: 10px; border: 1px solid;">
                <h1>{"Container C"}</h1>
            </div>

            <h1>{"zIndex and order"}</h1>
            <p>{"See squares in top-left"}</p>
            <Portal
                as_child={Callback::from(|_| html! {
                    <div style="width: 20px; height: 20px; background-color: red; position: absolute; top: 0px; left: 0px; z-index: 9999999;" />
                })}
            />
            <Portal
                as_child={Callback::from(|_| html! {
                    <div style="width: 20px; height: 20px; background-color: green; margin-left: 10px; margin-top: 10px; position: absolute; top: 0px; left: 0px; z-index: 9999999;" />
                })}
            />
            <Portal
                as_child={Callback::from(|_| html! {
                    <div style="width: 20px; height: 20px; background-color: blue; margin-left: 20px; margin-top: 20px; position: absolute; top: 0px; left: 0px; z-index: 9999999;" />
                })}
            />
        </div>
    }
}
