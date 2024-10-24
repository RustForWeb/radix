use radix_yew_themes::Box;
use yew::prelude::*;

use crate::decorative_box::DecorativeBox;

#[function_component]
pub fn BoxExample() -> Html {
    html! {
        <Box width="64px" height="64px">
            <DecorativeBox />
        </Box>
    }
}
