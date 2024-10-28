use radix_yew_themes::{Flex, FlexDirection, Link};
use yew::prelude::*;

#[function_component]
pub fn LinkSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Link href="#" size=1>
                {"Sign up"}
            </Link>
            <Link href="#" size=2>
                {"Sign up"}
            </Link>
            <Link href="#" size=3>
                {"Sign up"}
            </Link>
            <Link href="#" size=4>
                {"Sign up"}
            </Link>
            <Link href="#" size=5>
                {"Sign up"}
            </Link>
            <Link href="#" size=6>
                {"Sign up"}
            </Link>
            <Link href="#" size=7>
                {"Sign up"}
            </Link>
            <Link href="#" size=8>
                {"Sign up"}
            </Link>
            <Link href="#" size=9>
                {"Sign up"}
            </Link>
        </Flex>
    }
}
