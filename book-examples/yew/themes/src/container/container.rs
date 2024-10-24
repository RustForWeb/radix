use radix_yew_themes::{Box, Container};
use yew::prelude::*;

use crate::decorative_box::DecorativeBox;

#[function_component]
pub fn ContainerExample() -> Html {
    html! {
        <Box style={[("background", "var(--gray-a2)"), ("border-radius", "var(--radius-3)")]}>
            <Container size=1>
                <DecorativeBox>
                    <Box py=9 />
                </DecorativeBox>
            </Container>
        </Box>
    }
}
