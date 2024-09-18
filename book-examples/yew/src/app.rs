use yew::{prelude::*, virtual_dom::VNode};

#[function_component]
pub fn App() -> Html {
    let mut children: Vec<VNode> = vec![];

    #[cfg(feature = "separator")]
    {
        use crate::separator::SeparatorDemo;
        children.push(html! {
            <SeparatorDemo />
        });
    }

    html! {
        <div class="w-full h-full flex justify-center items-center">
            {children}
        </div>
    }
}
