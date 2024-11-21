use radix_yew_themes::{Container, Flex, FlexDirection, Skeleton, Text};
use yew::prelude::*;

#[function_component]
pub fn SkeletonWithTextExample() -> Html {
    html! {
        <Container size=1>
            <Flex direction={FlexDirection::Column} gap=2>
                <Text>
                    <Skeleton>{"Lorem ipsum dolor sit amet."}</Skeleton>
                </Text>

                <Skeleton>
                    <Text>{"Lorem ipsum dolor sit amet."}</Text>
                </Skeleton>
            </Flex>
        </Container>
    }
}
