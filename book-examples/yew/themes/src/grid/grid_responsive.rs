use radix_yew_themes::{Box, Breakpoint, Grid, ResponsiveValues};
use yew::prelude::*;

use crate::decorative_box::DecorativeBox;

#[function_component]
pub fn GridResponsiveExample() -> Html {
    html! {
        <Grid columns={ResponsiveValues::from([(Breakpoint::Initial, 1), (Breakpoint::Md, 2)])} gap=3 width="auto">
            <Box height="64px">
                <DecorativeBox />
            </Box>
            <Box height="64px">
                <DecorativeBox />
            </Box>
        </Grid>
    }
}
