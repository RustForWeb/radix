use radix_yew_themes::{AccentColor, Avatar, AvatarVariant, Grid, GridDisplay, GridFlow};
use yew::prelude::*;

#[function_component]
pub fn AvatarHighContrastExample() -> Html {
    html! {
        <Grid rows=2 gap=2 display={GridDisplay::InlineGrid} flow={GridFlow::Column}>
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Indigo} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Indigo} fallback="A" high_contrast=true />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Cyan} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Cyan} fallback="A" high_contrast=true />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Orange} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Orange} fallback="A" high_contrast=true />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Crimson} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Crimson} fallback="A" high_contrast=true />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Gray} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Gray} fallback="A" high_contrast=true />
        </Grid>
    }
}
