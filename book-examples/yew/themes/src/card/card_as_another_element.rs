use radix_yew_themes::{AccentColor, Box, Card, CardChildProps, Text, TextAs, Weight};
use yew::prelude::*;

#[function_component]
pub fn CardAsAnotherElementExample() -> Html {
    html! {
        <Box max_width="350px">
            <Card
                as_child={Callback::from(|CardChildProps { class, style, .. }| html! {
                    <a
                        class={class}
                        style={style}

                        href="#"
                    >
                        <Text r#as={TextAs::Div} size=2 weight={Weight::Bold}>
                            {"Quick start"}
                        </Text>
                        <Text r#as={TextAs::Div} color={AccentColor::Gray} size=2>
                            {"Start building your next project in minutes"}
                        </Text>
                    </a>
                })}
            />
        </Box>
    }
}
