use radix_yew_themes::{AccentColor, Theme};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    let mut children: Vec<Html> = vec![];

    #[cfg(feature = "aspect-ratio")]
    {
        use crate::aspect_ratio::{render, AspectRatioRoute};
        children.push(html! {
            <Switch<AspectRatioRoute> render={render} />
        });
    }
    #[cfg(feature = "avatar")]
    {
        use crate::avatar::{render, AvatarRoute};
        children.push(html! {
            <Switch<AvatarRoute> render={render} />
        });
    }
    #[cfg(feature = "blockquote")]
    {
        use crate::blockquote::{render, BlockquoteRoute};
        children.push(html! {
            <Switch<BlockquoteRoute> render={render} />
        });
    }
    #[cfg(feature = "box")]
    {
        use crate::r#box::{render, BoxRoute};
        children.push(html! {
            <Switch<BoxRoute> render={render} />
        });
    }
    #[cfg(feature = "button")]
    {
        use crate::button::{render, ButtonRoute};
        children.push(html! {
            <Switch<ButtonRoute> render={render} />
        });
    }
    #[cfg(feature = "code")]
    {
        use crate::code::{render, CodeRoute};
        children.push(html! {
            <Switch<CodeRoute> render={render} />
        });
    }
    #[cfg(feature = "container")]
    {
        use crate::container::{render, ContainerRoute};
        children.push(html! {
            <Switch<ContainerRoute> render={render} />
        });
    }
    #[cfg(feature = "em")]
    {
        use crate::em::{render, EmRoute};
        children.push(html! {
            <Switch<EmRoute> render={render} />
        });
    }
    #[cfg(feature = "flex")]
    {
        use crate::flex::{render, FlexRoute};
        children.push(html! {
            <Switch<FlexRoute> render={render} />
        });
    }
    #[cfg(feature = "grid")]
    {
        use crate::grid::{render, GridRoute};
        children.push(html! {
            <Switch<GridRoute> render={render} />
        });
    }
    #[cfg(feature = "heading")]
    {
        use crate::heading::{render, HeadingRoute};
        children.push(html! {
            <Switch<HeadingRoute> render={render} />
        });
    }
    #[cfg(feature = "icon-button")]
    {
        use crate::icon_button::{render, IconButtonRoute};
        children.push(html! {
            <Switch<IconButtonRoute> render={render} />
        });
    }
    #[cfg(feature = "kbd")]
    {
        use crate::kbd::{render, KbdRoute};
        children.push(html! {
            <Switch<KbdRoute> render={render} />
        });
    }
    #[cfg(feature = "link")]
    {
        use crate::link::{render, LinkRoute};
        children.push(html! {
            <Switch<LinkRoute> render={render} />
        });
    }
    #[cfg(feature = "quote")]
    {
        use crate::quote::{render, QuoteRoute};
        children.push(html! {
            <Switch<QuoteRoute> render={render} />
        });
    }
    #[cfg(feature = "section")]
    {
        use crate::section::{render, SectionRoute};
        children.push(html! {
            <Switch<SectionRoute> render={render} />
        });
    }
    #[cfg(feature = "select")]
    {
        use crate::select::{render, SelectRoute};
        children.push(html! {
            <div style="min-height: 300px;">
                <Switch<SelectRoute> render={render} />
            </div>
        });
    }
    #[cfg(feature = "separator")]
    {
        use crate::separator::{render, SeparatorRoute};
        children.push(html! {
            <Switch<SeparatorRoute> render={render} />
        });
    }
    #[cfg(feature = "spinner")]
    {
        use crate::spinner::{render, SpinnerRoute};
        children.push(html! {
            <Switch<SpinnerRoute> render={render} />
        });
    }
    #[cfg(feature = "strong")]
    {
        use crate::strong::{render, StrongRoute};
        children.push(html! {
            <Switch<StrongRoute> render={render} />
        });
    }
    #[cfg(feature = "switch")]
    {
        use crate::switch::{render, SwitchRoute};
        children.push(html! {
            <Switch<SwitchRoute> render={render} />
        });
    }
    #[cfg(feature = "text")]
    {
        use crate::text::{render, TextRoute};
        children.push(html! {
            <Switch<TextRoute> render={render} />
        });
    }
    #[cfg(feature = "text-area")]
    {
        use crate::text_area::{render, TextAreaRoute};
        children.push(html! {
            <Switch<TextAreaRoute> render={render} />
        });
    }
    #[cfg(feature = "text-field")]
    {
        use crate::text_field::{render, TextFieldRoute};
        children.push(html! {
            <Switch<TextFieldRoute> render={render} />
        });
    }

    html! {
        <Theme accent_color={AccentColor::Indigo}>
            <div style="padding: var(--space-4); line-height: 1;">
                <HashRouter>
                    {children}
                </HashRouter>
            </div>
        </Theme>
    }
}
