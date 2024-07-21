use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let children: Vec<VNode> = vec![];

    // if cfg!(feature = "avatar") {
    //     use crate::avatar::AvatarDemo;

    //     children.push(rsx! {
    //         AvatarDemo {}
    //     });
    // }

    rsx! {
        div {
            class: "w-full h-full flex justify-center items-center",
            {children.iter()}
        }
    }
}
