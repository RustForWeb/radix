use radix_yew_themes::{AccentColor, Flex, FlexDirection, Link};
use yew::prelude::*;

#[function_component]
pub fn LinkColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Link href="#" color={AccentColor::Indigo}>
                {"Sign up"}
            </Link>
            <Link href="#" color={AccentColor::Cyan}>
                {"Sign up"}
            </Link>
            <Link href="#" color={AccentColor::Orange}>
                {"Sign up"}
            </Link>
            <Link href="#" color={AccentColor::Crimson}>
                {"Sign up"}
            </Link>
        </Flex>
    }
}
