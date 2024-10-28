use radix_yew_themes::{Flex, FlexDirection, Link, LinkUnderline};
use yew::prelude::*;

#[function_component]
pub fn LinkUnderlineExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Link href="#" underline={LinkUnderline::Hover}>
                {"Sign up"}
            </Link>
            <Link href="#" underline={LinkUnderline::Always}>
                {"Sign up"}
            </Link>
        </Flex>
    }
}
