use radix_yew_themes::{AccentColor, Flex, FlexDirection, Link};
use yew::prelude::*;

#[function_component]
pub fn LinkHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Link color={AccentColor::Gray}>
                {"Sign up."}
            </Link>
            <Link href="#" color={AccentColor::Gray} high_contrast=true>
                {"Sign up"}
            </Link>
        </Flex>
    }
}
