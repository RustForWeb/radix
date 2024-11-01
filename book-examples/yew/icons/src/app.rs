use yew::{prelude::*, virtual_dom::VNode};

#[function_component]
pub fn App() -> Html {
    #[allow(unused_mut)]
    let mut children: Vec<VNode> = vec![];

    #[cfg(feature = "icons")]
    {
        use crate::icons::IconsDemo;
        children.push(html! { <IconsDemo /> });
    }

    html! { <div class="w-full h-full flex justify-center items-start">{ children }</div> }
}
