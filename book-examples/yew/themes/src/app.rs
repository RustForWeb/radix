use radix_yew_themes::{AccentColor, Theme};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    let mut children: Vec<Html> = vec![];

    #[cfg(feature = "aspect-ratio")]
    {
        use crate::aspect_ratio::{AspectRatioRoute, render};
        children.push(html! {
            <Switch<AspectRatioRoute> render={render} />
        });
    }
    #[cfg(feature = "avatar")]
    {
        use crate::avatar::{AvatarRoute, render};
        children.push(html! {
            <Switch<AvatarRoute> render={render} />
        });
    }
    #[cfg(feature = "badge")]
    {
        use crate::badge::{BadgeRoute, render};
        children.push(html! {
            <Switch<BadgeRoute> render={render} />
        });
    }
    #[cfg(feature = "blockquote")]
    {
        use crate::blockquote::{BlockquoteRoute, render};
        children.push(html! {
            <Switch<BlockquoteRoute> render={render} />
        });
    }
    #[cfg(feature = "box")]
    {
        use crate::r#box::{BoxRoute, render};
        children.push(html! {
            <Switch<BoxRoute> render={render} />
        });
    }
    #[cfg(feature = "button")]
    {
        use crate::button::{ButtonRoute, render};
        children.push(html! {
            <Switch<ButtonRoute> render={render} />
        });
    }
    #[cfg(feature = "callout")]
    {
        use crate::callout::{CalloutRoute, render};
        children.push(html! {
            <Switch<CalloutRoute> render={render} />
        });
    }
    #[cfg(feature = "card")]
    {
        use crate::card::{CardRoute, render};
        children.push(html! {
            <Switch<CardRoute> render={render} />
        });
    }
    #[cfg(feature = "checkbox")]
    {
        use crate::checkbox::{CheckboxRoute, render};
        children.push(html! {
            <Switch<CheckboxRoute> render={render} />
        });
    }
    #[cfg(feature = "code")]
    {
        use crate::code::{CodeRoute, render};
        children.push(html! {
            <Switch<CodeRoute> render={render} />
        });
    }
    #[cfg(feature = "container")]
    {
        use crate::container::{ContainerRoute, render};
        children.push(html! {
            <Switch<ContainerRoute> render={render} />
        });
    }
    #[cfg(feature = "data-list")]
    {
        use crate::data_list::{DataListRoute, render};
        children.push(html! {
            <Switch<DataListRoute> render={render} />
        });
    }
    #[cfg(feature = "em")]
    {
        use crate::em::{EmRoute, render};
        children.push(html! {
            <Switch<EmRoute> render={render} />
        });
    }
    #[cfg(feature = "flex")]
    {
        use crate::flex::{FlexRoute, render};
        children.push(html! {
            <Switch<FlexRoute> render={render} />
        });
    }
    #[cfg(feature = "grid")]
    {
        use crate::grid::{GridRoute, render};
        children.push(html! {
            <Switch<GridRoute> render={render} />
        });
    }
    #[cfg(feature = "heading")]
    {
        use crate::heading::{HeadingRoute, render};
        children.push(html! {
            <Switch<HeadingRoute> render={render} />
        });
    }
    #[cfg(feature = "icon-button")]
    {
        use crate::icon_button::{IconButtonRoute, render};
        children.push(html! {
            <Switch<IconButtonRoute> render={render} />
        });
    }
    #[cfg(feature = "inset")]
    {
        use crate::inset::{InsetRoute, render};
        children.push(html! {
            <Switch<InsetRoute> render={render} />
        });
    }
    #[cfg(feature = "kbd")]
    {
        use crate::kbd::{KbdRoute, render};
        children.push(html! {
            <Switch<KbdRoute> render={render} />
        });
    }
    #[cfg(feature = "link")]
    {
        use crate::link::{LinkRoute, render};
        children.push(html! {
            <Switch<LinkRoute> render={render} />
        });
    }
    #[cfg(feature = "quote")]
    {
        use crate::quote::{QuoteRoute, render};
        children.push(html! {
            <Switch<QuoteRoute> render={render} />
        });
    }
    #[cfg(feature = "radio")]
    {
        use crate::radio::{RadioRoute, render};
        children.push(html! {
            <Switch<RadioRoute> render={render} />
        });
    }
    #[cfg(feature = "section")]
    {
        use crate::section::{SectionRoute, render};
        children.push(html! {
            <Switch<SectionRoute> render={render} />
        });
    }
    #[cfg(feature = "select")]
    {
        use crate::select::{SelectRoute, render};
        children.push(html! {
            <div style="min-height: 300px;">
                <Switch<SelectRoute> render={render} />
            </div>
        });
    }
    #[cfg(feature = "separator")]
    {
        use crate::separator::{SeparatorRoute, render};
        children.push(html! {
            <Switch<SeparatorRoute> render={render} />
        });
    }
    #[cfg(feature = "skeleton")]
    {
        use crate::skeleton::{SkeletonRoute, render};
        children.push(html! {
            <Switch<SkeletonRoute> render={render} />
        });
    }
    #[cfg(feature = "spinner")]
    {
        use crate::spinner::{SpinnerRoute, render};
        children.push(html! {
            <Switch<SpinnerRoute> render={render} />
        });
    }
    #[cfg(feature = "strong")]
    {
        use crate::strong::{StrongRoute, render};
        children.push(html! {
            <Switch<StrongRoute> render={render} />
        });
    }
    #[cfg(feature = "switch")]
    {
        use crate::switch::{SwitchRoute, render};
        children.push(html! {
            <Switch<SwitchRoute> render={render} />
        });
    }
    #[cfg(feature = "table")]
    {
        use crate::table::{TableRoute, render};
        children.push(html! {
            <Switch<TableRoute> render={render} />
        });
    }
    #[cfg(feature = "text")]
    {
        use crate::text::{TextRoute, render};
        children.push(html! {
            <Switch<TextRoute> render={render} />
        });
    }
    #[cfg(feature = "text-area")]
    {
        use crate::text_area::{TextAreaRoute, render};
        children.push(html! {
            <Switch<TextAreaRoute> render={render} />
        });
    }
    #[cfg(feature = "text-field")]
    {
        use crate::text_field::{TextFieldRoute, render};
        children.push(html! {
            <Switch<TextFieldRoute> render={render} />
        });
    }
    #[cfg(feature = "tooltip")]
    {
        use crate::tooltip::{TooltipRoute, render};
        children.push(html! {
            <Switch<TooltipRoute> render={render} />
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
