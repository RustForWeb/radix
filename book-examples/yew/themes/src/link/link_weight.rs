use radix_yew_themes::{Flex, FlexDirection, Link, Weight};
use yew::prelude::*;

#[function_component]
pub fn LinkWeightExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Link href="#" weight={Weight::Light}>
                {"Sign up"}
            </Link>
            <Link href="#" weight={Weight::Regular}>
                {"Sign up"}
            </Link>
            <Link href="#" weight={Weight::Medium}>
                {"Sign up"}
            </Link>
            <Link href="#" weight={Weight::Bold}>
                {"Sign up"}
            </Link>
        </Flex>
    }
}
