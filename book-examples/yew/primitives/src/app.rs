use yew::{prelude::*, virtual_dom::VNode};

#[function_component]
pub fn App() -> Html {
    let mut children: Vec<VNode> = vec![];

    #[cfg(feature = "icons")]
    {
        use crate::icons::IconsDemo;
        children.push(html! {
            <IconsDemo />
        });
    }
    #[cfg(feature = "label")]
    {
        use crate::label::LabelDemo;
        children.push(html! {
            <LabelDemo />
        });
    }
    #[cfg(feature = "select")]
    {
        use crate::select::SelectDemo;
        children.push(html! {
            <SelectDemo />
        });
    }
    #[cfg(feature = "separator")]
    {
        use crate::separator::SeparatorDemo;
        children.push(html! {
            <SeparatorDemo />
        });
    }
    #[cfg(feature = "switch")]
    {
        use crate::switch::SwitchDemo;
        children.push(html! {
            <SwitchDemo />
        });
    }

    html! {
        <div class="w-full h-full flex justify-center items-start">
            {children}
        </div>
    }
}
