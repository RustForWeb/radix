use radix_yew_themes::{AccentColor, Checkbox, CheckedState, Grid, GridDisplay};
use yew::prelude::*;

#[function_component]
pub fn CheckboxHighContrastExample() -> Html {
    html! {
        <Grid columns=5 display={GridDisplay::InlineGrid} gap=2>
            <Checkbox color={AccentColor::Indigo} default_checked={CheckedState::True} />
            <Checkbox color={AccentColor::Cyan} default_checked={CheckedState::True} />
            <Checkbox color={AccentColor::Orange} default_checked={CheckedState::True} />
            <Checkbox color={AccentColor::Crimson} default_checked={CheckedState::True} />
            <Checkbox color={AccentColor::Gray} default_checked={CheckedState::True} />

            <Checkbox color={AccentColor::Indigo} default_checked={CheckedState::True} high_contrast=true />
            <Checkbox color={AccentColor::Cyan} default_checked={CheckedState::True} high_contrast=true />
            <Checkbox color={AccentColor::Orange} default_checked={CheckedState::True} high_contrast=true />
            <Checkbox color={AccentColor::Crimson} default_checked={CheckedState::True} high_contrast=true />
            <Checkbox color={AccentColor::Gray} default_checked={CheckedState::True} high_contrast=true />
        </Grid>
    }
}
