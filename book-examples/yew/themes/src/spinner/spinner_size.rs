use radix_yew_themes::{Flex, FlexAlign, Spinner};
use yew::prelude::*;

#[function_component]
pub fn SpinnerSizeExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=4>
            <Spinner size=1 />
            <Spinner size=2 />
            <Spinner size=3 />
        </Flex>
    }
}
