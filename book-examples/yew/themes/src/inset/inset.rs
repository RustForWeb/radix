use radix_yew_themes::{
    Box, Card, Inset, InsetClip, InsetPadding, InsetSide, Strong, Text, TextAs,
};
use yew::prelude::*;
use yew_style::Style;

#[function_component]
pub fn InsetExample() -> Html {
    html! {
        <Box max_width="240px">
            <Card size=2>
                <Inset clip={InsetClip::PaddingBox} side={InsetSide::Top} pb={InsetPadding::Current}>
                    <img
                        src="https://images.unsplash.com/photo-1617050318658-a9a3175e34cb?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=600&q=80"
                        alt="Bold typography"
                        style={Style::from([
                            ("display", "block"),
                            ("object-fit", "cover"),
                            ("width", "100%"),
                            ("height", "140px"),
                            ("background-color", "var(--gray-5)"),
                        ])}
                    />
                </Inset>
                <Text r#as={TextAs::P} size=3>
                    <Strong>{"Typography"}</Strong>{" is the art and technique of arranging type to
                    make written language legible, readable and appealing when displayed."}
                </Text>
            </Card>
        </Box>
    }
}
