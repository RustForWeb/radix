use yew::{prelude::*, virtual_dom::VNode};

#[function_component]
pub fn App() -> Html {
    let children: Vec<VNode> = vec![];

    // if cfg!(feature = "avatar") {
    //     use crate::avatar::AvatarDemo;

    //     children.push(html! {
    //         <AvatarDemo />
    //     });
    // }

    html! {
        <div class="w-full h-full flex justify-center items-center">
            {children}
        </div>
    }
}
