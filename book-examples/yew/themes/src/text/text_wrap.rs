use radix_yew_themes::{Flex, FlexDirection, Text, TextWrap};
use yew::prelude::*;

#[function_component]
pub fn TextWrapExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Flex max_width="270px">
                <Text wrap={TextWrap::Nowrap}>
                    {"The goal of typography is to relate font size, line height, and line width
                    in a proportional way that maximizes beauty and makes reading easier and
                    more pleasant."}
                </Text>
            </Flex>

            <Flex max_width="270px">
                <Text wrap={TextWrap::Balance}>
                    {"The goal of typography is to relate font size, line height, and line width
                    in a proportional way that maximizes beauty and makes reading easier and
                    more pleasant."}
                </Text>
            </Flex>

            <Flex max_width="270px">
                <Text wrap={TextWrap::Pretty}>
                    {"The goal of typography is to relate font size, line height, and line width
                    in a proportional way that maximizes beauty and makes reading easier and
                    more pleasant."}
                </Text>
            </Flex>
        </Flex>
    }
}
