use radix_yew_themes::{Flex, Strong};
use yew::prelude::*;

#[function_component]
pub fn StrongTruncateExample() -> Html {
    html! {
        <Flex max_width="300px">
            <Strong truncate=true>
                {"The goal of typography is to relate font size, line height, and line width
                in a proportional way that maximizes beauty and makes reading easier and
                more pleasant."}
            </Strong>
        </Flex>
    }
}
