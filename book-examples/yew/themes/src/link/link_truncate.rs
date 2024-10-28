use radix_yew_themes::{Flex, Link};
use yew::prelude::*;

#[function_component]
pub fn LinkTruncateExample() -> Html {
    html! {
        <Flex max_width="150px">
            <Link href="#" truncate=true>
                {"Sign up to the newsletter"}
            </Link>
        </Flex>
    }
}
