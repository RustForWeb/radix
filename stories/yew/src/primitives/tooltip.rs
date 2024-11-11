use radix_yew_tooltip::*;
use tailwind_fuse::*;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    let trigger_class = use_memo((), |_| TriggerClass::default().to_class());
    let content_class = use_memo((), |_| ContentClass::default().to_class());
    let arrow_class = use_memo((), |_| ArrowClass::default().to_class());

    html! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger class={(*trigger_class).clone()}>{"Hover or Focus me"}</TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent class={(*content_class).clone()} side_offset={5.0}>
                        {"Nicely done!"}
                        <TooltipArrow class={(*arrow_class).clone()} />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}

#[function_component]
pub fn Controlled() -> Html {
    html! {}
}

#[function_component]
pub fn CustomDurations() -> Html {
    html! {}
}

#[function_component]
pub fn CustomContent() -> Html {
    html! {}
}

#[function_component]
pub fn Positions() -> Html {
    html! {}
}

#[function_component]
pub fn AriaLabel() -> Html {
    html! {}
}

#[function_component]
pub fn WithText() -> Html {
    html! {}
}

#[function_component]
pub fn WithExternalRef() -> Html {
    html! {}
}

#[function_component]
pub fn Unmount() -> Html {
    html! {}
}

#[function_component]
pub fn Animated() -> Html {
    html! {}
}

#[function_component]
pub fn SlottableContent() -> Html {
    html! {}
}

#[function_component]
pub fn WithinDialog() -> Html {
    html! {}
}

#[function_component]
pub fn KeepOpenOnActivation() -> Html {
    html! {}
}

#[function_component]
pub fn WithinScrollable() -> Html {
    html! {}
}

#[function_component]
pub fn DisableHoverableContent() -> Html {
    html! {}
}

#[function_component]
pub fn Chromatic() -> Html {
    html! {}
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct TriggerClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "origin-[var(--radix-tooltip-content-transform-origin)] select-none bg-[#111] text-[#fff] text-[12px] rounded-[5px] p-[10px] max-w-[300px]"
)]
struct ContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "fill-[#111]")]
struct ArrowClass {}
