use radix_yew_themes::{AccentColor, Grid, GridDisplay, Radio};
use yew::prelude::*;

#[function_component]
pub fn RadioHighContrastExample() -> Html {
    html! {
        <Grid columns=5 display={GridDisplay::InlineGrid} gap=2>
            <Radio color={AccentColor::Indigo} default_checked=true />
            <Radio color={AccentColor::Cyan} default_checked=true />
            <Radio color={AccentColor::Orange} default_checked=true />
            <Radio color={AccentColor::Crimson} default_checked=true />
            <Radio color={AccentColor::Gray} default_checked=true />

            <Radio color={AccentColor::Indigo} default_checked=true high_contrast=true />
            <Radio color={AccentColor::Cyan} default_checked=true high_contrast=true />
            <Radio color={AccentColor::Orange} default_checked=true high_contrast=true />
            <Radio color={AccentColor::Crimson} default_checked=true high_contrast=true />
            <Radio color={AccentColor::Gray} default_checked=true high_contrast=true />
        </Grid>
    }
}
