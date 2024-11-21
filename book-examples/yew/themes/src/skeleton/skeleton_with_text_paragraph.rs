use radix_yew_themes::{Container, Flex, FlexDirection, Skeleton, Text};
use yew::prelude::*;

#[function_component]
pub fn SkeletonWithTextParagraphExample() -> Html {
    html! {
        <Container size=1>
            <Flex direction={FlexDirection::Column} gap=2>
                <Text>
                    <Skeleton>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque
                        felis tellus, efficitur id convallis a, viverra eget libero. Nam magna
                        erat, fringilla sed commodo sed, aliquet nec magna."}
                    </Skeleton>
                </Text>

                <Skeleton>
                    <Text>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque
                        felis tellus, efficitur id convallis a, viverra eget libero. Nam magna
                        erat, fringilla sed commodo sed, aliquet nec magna."}
                    </Text>
                </Skeleton>
            </Flex>
        </Container>
    }
}
