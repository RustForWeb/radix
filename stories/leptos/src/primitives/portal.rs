use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_portal::Portal;

#[component]
pub fn Base() -> impl IntoView {
    view! {
        <div
            style:max-width="300px"
            style:max-height="200px"
            style:overflow="auto"
            style:border="1px solid"
        >
            <h1>This content is rendered in the main DOM tree</h1>
            <p>
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos porro, est ex quia itaque facere
                fugit necessitatibus aut enim. Nisi rerum quae, repellat in perspiciatis explicabo laboriosam
                necessitatibus eius pariatur.
            </p>

            <Portal>
                <h1>This content is rendered in a portal (another DOM tree)</h1>
                <p>
                    Because of the portal, it can appear in a different DOM tree from the main one (by default a
                    new element inside the body), even though it is part of the same Leptos tree.
                </p>
            </Portal>
        </div>
    }
}

#[component]
pub fn CustomContainer() -> impl IntoView {
    let portal_container_ref = AnyNodeRef::new();

    view! {
        <div
            style:max-width="300px"
            style:padding="10px"
            style:margin="10px"
            style:border="1px solid"
        >
            <h1>Container A</h1>
            <Portal as_child=true container_ref=portal_container_ref>
                <p>
                    This content is rendered in a portal inside Container A but appears inside Container B
                    because we have used Container B as a container element for the Portal.
                </p>
            </Portal>
        </div>

        <div
            node_ref=portal_container_ref
            style:max-width="300px"
            style:padding="10px"
            style:margin="10px"
            style:border="1px solid"
        >
            <h1>Container B</h1>
        </div>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let portal_container_ref = AnyNodeRef::new();

    view! {
        <div style:padding="150px">
            <h1>Default (append to body)</h1>
            <div
                style:padding="10px"
                style:margin="10px"
                style:border="1px solid blue"
            >
                <p>Container A</p>

                <Portal as_child=true>
                    <div
                        style:padding="10px"
                        style:margin="10px"
                        style:border="1px solid blue"
                        style:position="absolute"
                        style:top="0px"
                        style:left="0px"
                        style:z-index="9999999"
                    >
                        <p>This content is rendered in a portal (another DOM tree)</p>
                        <p>
                            Because of the portal, it can appear in a different DOM tree from the main one (by
                            default a new element inside the body), even though it is part of the same Leptos tree.
                        </p>
                    </div>
                </Portal>
            </div>

            <h1>Custom container</h1>
            <div
                style:padding="10px"
                style:margin="10px"
                style:border="1px solid green"
            >
                <h1>Container B</h1>
                <Portal as_child=true container_ref=portal_container_ref>
                    <div
                        style:padding="10px"
                        style:margin="10px"
                        style:border="1px solid green"
                    >
                        <p>
                            This content is rendered in a portal inside Container B but appears inside Container C
                            because we have used Container C as a container element for the Portal.
                        </p>
                    </div>
                </Portal>
            </div>

            <div
                node_ref=portal_container_ref
                style:padding="10px"
                style:margin="10px"
                style:border="1px solid"
            >
                <h1>Container C</h1>
            </div>

            <h1>zIndex and order</h1>
            <p>See squares in top-left</p>
            <Portal as_child=true>
                <div
                    style:width="20px"
                    style:height="20px"
                    style:background-color="red"
                    style:position="absolute"
                    style:top="0px"
                    style:left="0px"
                    style:z-index="9999999"
                />
            </Portal>
            <Portal as_child=true>
                <div
                    style:width="20px"
                    style:height="20px"
                    style:background-color="green"
                    style:margin-left="10px"
                    style:margin-top="10px"
                    style:position="absolute"
                    style:top="0px"
                    style:left="0px"
                    style:z-index="9999999"
                />
            </Portal>
            <Portal as_child=true>
                <div
                    style:width="20px"
                    style:height="20px"
                    style:background-color="blue"
                    style:margin-left="20px"
                    style:margin-top="20px"
                    style:position="absolute"
                    style:top="0px"
                    style:left="0px"
                    style:z-index="9999999"
                />
            </Portal>
        </div>
    }
}
