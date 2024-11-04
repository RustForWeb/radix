use radix_yew_themes::{Flex, Spinner, Switch};
use yew::prelude::*;

#[function_component]
pub fn SpinnerWithChildrenExample() -> Html {
    html! {
        <Flex gap=4>
            <Spinner loading={true}>
                <Switch default_checked=true />
            </Spinner>

            <Spinner loading={false}>
                <Switch default_checked=true />
            </Spinner>
        </Flex>
    }
}
