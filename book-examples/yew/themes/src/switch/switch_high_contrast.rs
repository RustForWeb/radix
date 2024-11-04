use radix_yew_themes::{AccentColor, Grid, GridDisplay, GridFlow, Switch};
use yew::prelude::*;

#[function_component]
pub fn SwitchHighContrastExample() -> Html {
    html! {
        <Grid rows=2 gap_x=2 gap_y=3 display={GridDisplay::InlineGrid} flow={GridFlow::Column}>
            <Switch color={AccentColor::Indigo} default_checked=true />
            <Switch color={AccentColor::Indigo} default_checked=true high_contrast=true />
            <Switch color={AccentColor::Cyan} default_checked=true />
            <Switch color={AccentColor::Cyan} default_checked=true high_contrast=true />
            <Switch color={AccentColor::Orange} default_checked=true />
            <Switch color={AccentColor::Orange} default_checked=true high_contrast=true />
            <Switch color={AccentColor::Crimson} default_checked=true />
            <Switch color={AccentColor::Crimson} default_checked=true high_contrast=true />
            <Switch color={AccentColor::Gray} default_checked=true />
            <Switch color={AccentColor::Gray} default_checked=true high_contrast=true />
        </Grid>
    }
}
