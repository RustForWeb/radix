use radix_yew_themes::Grid;
use yew::prelude::*;

use crate::decorative_box::DecorativeBox;

#[function_component]
pub fn GridExample() -> Html {
    html! {
        <Grid columns=3 gap=3 rows="repeat(2, 64px)" width="auto">
            <DecorativeBox />
            <DecorativeBox />
            <DecorativeBox />
            <DecorativeBox />
            <DecorativeBox />
            <DecorativeBox />
        </Grid>
    }
}
