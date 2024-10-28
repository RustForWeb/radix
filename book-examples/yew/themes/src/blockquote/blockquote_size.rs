use radix_yew_themes::{Blockquote, Box, Flex, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn BlockquoteSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=5>
            <Box max_width="300px">
                <Blockquote size=1>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="350px">
                <Blockquote size=2>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="400px">
                <Blockquote size=3>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="450px">
                <Blockquote size=4>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="500px">
                <Blockquote size=5>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="550px">
                <Blockquote size=6>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="600px">
                <Blockquote size=7>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="650px">
                <Blockquote size=8>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
            <Box max_width="1000px">
                <Blockquote size=9>
                    {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                    stone alone comes near it in obstinacy."}
                </Blockquote>
            </Box>
        </Flex>
    }
}
