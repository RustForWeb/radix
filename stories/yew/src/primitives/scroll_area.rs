// TODO: Remove
#![allow(dead_code)]

use radix_yew_scroll_area::*;
use tailwind_fuse::*;
use yew::prelude::*;
use yew_style::Style;

#[function_component]
pub fn Basic() -> Html {
    html! {}
}

#[function_component]
pub fn Resizable() -> Html {
    html! {}
}

#[function_component]
pub fn ContentChange() -> Html {
    html! {}
}

#[function_component]
pub fn Animated() -> Html {
    html! {}
}

#[function_component]
pub fn Chromatic() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticDynamicContentBeforeLoaded() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticDynamicContentAfterLoaded() -> Html {
    html! {}
}

#[derive(PartialEq, Properties)]
struct ScrollAreaStoryProps {
    #[prop_or(false)]
    pub animated: bool,
    #[prop_or(true)]
    pub vertical: bool,
    #[prop_or(true)]
    pub horizontal: bool,

    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn ScrollAreaStory(props: &ScrollAreaStoryProps) -> Html {
    let scroll_area_class = use_memo((), |_| ScrollAreaClass::default().to_class());
    let scroll_area_viewport_class =
        use_memo((), |_| ScrollAreaViewportClass::default().to_class());
    let scrollbar_class = use_memo((), |_| ScrollbarClass::default().to_class());
    let animated_thumb_class = use_memo((), |_| AnimatedThumbClass::default().to_class());
    let thumb_class = use_memo((), |_| ThumbClass::default().to_class());
    let corner_class = use_memo((), |_| CornerClass::default().to_class());

    html! {
        <ScrollArea
            class={(*scroll_area_class).clone()}
            style={props.style.clone().with_defaults([
                ("width", "200px"),
                ("height", "200px"),
            ])}
        >
            <ScrollAreaViewport class={(*scroll_area_viewport_class).clone()}>{props.children.clone()}</ScrollAreaViewport>
            if props.vertical {
                <ScrollAreaScrollbar class={(*scrollbar_class).clone()} orientation={ScrollAreaOrientation::Vertical}>
                    <ScrollAreaThumb class={if props.animated { (*animated_thumb_class).clone() } else { (*thumb_class).clone() }} />
                </ScrollAreaScrollbar>
            }
            if props.horizontal {
                <ScrollAreaScrollbar class={(*scrollbar_class).clone()} orientation={ScrollAreaOrientation::Horizontal}>
                    <ScrollAreaThumb class={if props.animated { (*animated_thumb_class).clone() } else { (*thumb_class).clone() }} />
                </ScrollAreaScrollbar>
            }
            <ScrollAreaCorner class={(*corner_class).clone()} />
        </ScrollArea>
    }
}

#[derive(PartialEq, Properties)]
struct CopyProps {
    #[prop_or_default]
    pub style: Style,
}

#[function_component]
fn Copy(props: &CopyProps) -> Html {
    html! {
        <p
            style={props.style.clone().with_defaults([
                ("width", "4000px"),
                ("margin-top", "0px"),
            ])}
        >
            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce sit amet eros iaculis, bibendum
            tellus ac, lobortis odio. Aliquam bibendum elit est, in iaculis est commodo id. Donec pulvinar
            est libero. Proin consectetur pellentesque molestie. Fusce mi ante, ullamcorper eu ante finibus,
            finibus pellentesque turpis. Mauris convallis, leo in vulputate varius, sapien lectus suscipit
            eros, ac semper odio sapien sit amet magna. Sed mattis turpis et lacinia ultrices. Nulla a
            commodo mauris. Orci varius natoque penatibus et magnis dis parturient montes, nascetur
            ridiculus mus. Pellentesque id tempor metus. Pellentesque faucibus tortor non nisi maximus
            dignissim. Etiam leo nisi, molestie a porttitor at, euismod a libero. Nullam placerat tristique
            enim nec pulvinar. Sed eleifend dictum nulla a aliquam. Sed tempus ipsum eget urna posuere
            aliquam. Nulla maximus tortor dui, sed laoreet odio aliquet ac. Vestibulum dolor orci, lacinia
            finibus vehicula eget, posuere ac lectus. Quisque non felis at ipsum scelerisque condimentum. In
            pharetra semper arcu, ut hendrerit sem auctor vel. Aliquam non lacinia elit, a facilisis ante.
            Praesent eget eros augue. Praesent nunc orci, ullamcorper non pulvinar eu, elementum id nibh.
            Nam id lorem euismod, sodales augue quis, porttitor magna. Vivamus ut nisl velit. Nam ultrices
            maximus felis, quis ullamcorper quam luctus et."}
      </p>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct ScrollAreaClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct ScrollAreaViewportClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct ScrollbarClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct ThumbClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct CornerClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct AnimatedThumbClass {}
