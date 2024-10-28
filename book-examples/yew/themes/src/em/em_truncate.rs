use radix_yew_themes::{Em, Flex};
use yew::prelude::*;

#[function_component]
pub fn EmTruncateExample() -> Html {
    html! {
        <Flex max_width="300px">
            <Em truncate=true>
                {"The goal of typography is to relate font size, line height, and line width
                in a proportional way that maximizes beauty and makes reading easier and
                more pleasant."}
            </Em>
        </Flex>
    }
}
