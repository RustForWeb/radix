use radix_yew_icons::{DotsHorizontalIcon, MagnifyingGlassIcon};
use radix_yew_themes::{
    Box, Flex, FlexDirection, IconButton, IconButtonVariant, TextField, TextFieldSlot,
};
use yew::prelude::*;

#[function_component]
pub fn TextFieldSizeButtonsExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="400px">
            <Box max_width="200px">
                <TextField placeholder="Search the docs…" size=1>
                    <TextFieldSlot>
                        <MagnifyingGlassIcon height=16 width=16 />
                    </TextFieldSlot>
                </TextField>
            </Box>

            <Box max_width="250px">
                <TextField placeholder="Search the docs…" size=2>
                    <TextFieldSlot>
                        <MagnifyingGlassIcon height=16 width=16 />
                    </TextFieldSlot>
                    <TextFieldSlot>
                        <IconButton size=1 variant={IconButtonVariant::Ghost}>
                            <DotsHorizontalIcon height=14 width=14 />
                        </IconButton>
                    </TextFieldSlot>
                </TextField>
            </Box>

            <Box max_width="300px">
                <TextField placeholder="Search the docs…" size=3>
                    <TextFieldSlot>
                        <MagnifyingGlassIcon height=16 width=16 />
                    </TextFieldSlot>
                    <TextFieldSlot pr=3>
                        <IconButton size=2 variant={IconButtonVariant::Ghost}>
                            <DotsHorizontalIcon height=16 width=16 />
                        </IconButton>
                    </TextFieldSlot>
                </TextField>
            </Box>
        </Flex>
    }
}
