use radix_yew_label::Label;
use radix_yew_select::{Position as PositionEnum, *};
use tailwind_fuse::*;
use yew::prelude::*;
use yew_attrs::attrs;

const POSITIONS: [PositionEnum; 2] = [PositionEnum::ItemAligned, PositionEnum::Popper];

#[function_component]
pub fn Styled() -> Html {
    let trigger_class = use_memo((), |_| TriggerClass::default().to_class());
    let content_class = use_memo((), |_| ContentClass::default().to_class());
    let viewport_class = use_memo((), |_| ViewportClass::default().to_class());
    let item_class = use_memo((), |_| ItemClass::default().to_class());
    let indicator_class = use_memo((), |_| IndicatorClass::default().to_class());

    html! {
        <div style="display: flex; gap: 20px; padding: 50px;">
            {POSITIONS.iter().map(|position| html! {
                <Label key={position.to_string()}>
                    {"Choose a number:"}
                    <Select default_value="two">
                        <SelectTrigger attrs={attrs! { class={(*trigger_class).clone()} }}>
                            <SelectValue />
                            <SelectIcon />
                        </SelectTrigger>
                        <SelectPortal>
                            <SelectContent position={*position} attrs={attrs! { class={(*content_class).clone()} }}>
                                <SelectViewport attrs={attrs! { class={(*viewport_class).clone()} }}>
                                    <SelectItem value="one" attrs={attrs! { class={(*item_class).clone()} }}>
                                        <SelectItemText>
                                            {"One"}<span aria-hidden="">{" 👍"}</span>
                                        </SelectItemText>
                                        <SelectItemIndicator attrs={attrs! { class={(*indicator_class).clone()} }}>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                    <SelectItem value="two" attrs={attrs! { class={(*item_class).clone()} }}>
                                        <SelectItemText>
                                            {"Two"}<span aria-hidden="">{" 👌"}</span>
                                        </SelectItemText>
                                        <SelectItemIndicator attrs={attrs! { class={(*indicator_class).clone()} }}>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                    <SelectItem value="three" attrs={attrs! { class={(*item_class).clone()} }}>
                                        <SelectItemText>
                                            {"Three"}<span aria-hidden="">{" 🤘"}</span>
                                        </SelectItemText>
                                        <SelectItemIndicator attrs={attrs! { class={(*indicator_class).clone()} }}>
                                            <TickIcon />
                                        </SelectItemIndicator>
                                    </SelectItem>
                                </SelectViewport>
                            </SelectContent>
                        </SelectPortal>
                    </Select>
                </Label>
            }).collect::<Html>()}
        </div>
    }
}

#[function_component]
pub fn Controlled() -> Html {
    html! {}
}

#[function_component]
pub fn Position() -> Html {
    html! {}
}

#[function_component]
pub fn NoDefaultValue() -> Html {
    html! {}
}

#[function_component]
pub fn Typeahead() -> Html {
    html! {}
}

#[function_component]
pub fn WithGroups() -> Html {
    html! {}
}

#[function_component]
pub fn Labelling() -> Html {
    html! {}
}

#[function_component]
pub fn RightToLeft() -> Html {
    html! {}
}

#[function_component]
pub fn WithinForm() -> Html {
    html! {}
}

#[function_component]
pub fn DisabledWithinForm() -> Html {
    html! {}
}

#[function_component]
pub fn RequiredWithForm() -> Html {
    html! {}
}

#[function_component]
pub fn WithinDialog() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticShortOptionsPaddedContent() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticShortOptionsPaddedViewport() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticLongOptionsPaddedContent() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticLongOptionsPaddedViewport() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticTopFirstPaddedContent() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticTopFirstPaddedViewport() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticBottomLastPaddedContent() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticBottomLastPaddedViewport() -> Html {
    html! {}
}

#[function_component]
pub fn ChromaticNoDefaultValue() -> Html {
    html! {}
}

#[function_component]
pub fn Cypress() -> Html {
    html! {}
}

#[function_component]
fn TickIcon() -> Html {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 32 32"
            width="12"
            height="12"
            fill="none"
            stroke="currentcolor"
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="3"
        >
            <path d="M2 20 L12 28 30 4" />
        </svg>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center gap-[5px] border-[1px] border-solid border-[#111] rounded-[6px] bg-transparent h-[50px] p-[5px_15px] font-['apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif'] text-[13px] leading-none focus:outline-none focus:shadow-[0_0_0_2px_rgba(0,0,0,0.5)]"
)]
pub struct TriggerClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[#fff] border-[1px] border-solid border-[#ccc] rounded-[6px] relative focus-within:border-[#111] min-w-[var(--radix-select-trigger-width)] max-w-[var(--radix-select-content-available-height)]"
)]
pub struct ContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[#fff] border-[1px] border-solid border-[#ccc] rounded-[6px] relative focus-within:border-[#111] min-w-[var(--radix-select-trigger-width)] max-w-[var(--radix-select-content-available-height)] p-[5px]"
)]
pub struct ContentWithPaddingClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "p-[5px]")]
pub struct ViewportClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_25px] font-['apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif'] text-[13px] rounded-[3px] text-[#aaa] font-medium"
)]
pub struct LabelClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_25px] font-['apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif'] text-[13px] text-[#111] rounded-[3px] relative outline-none active:bg-[#ccc] data-[highlighted]:bg-[#111] data-[highlighted]:text-[#fff] data-[disabled]:text-[#ccc] rtl:text-[16px] rtl:font-bold"
)]
pub struct ItemClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_25px] font-['apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif'] text-[13px] text-[#111] rounded-[3px] relative outline-none active:bg-[#ccc] data-[highlighted]:bg-[#111] data-[highlighted]:text-[#fff] data-[disabled]:text-[#ccc] rtl:text-[16px] rtl:font-bold pl-[35px]"
)]
pub struct ItemInGroupClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "absolute left-[6px] top-[6px] rtl:left-auto rtl:right-[6px]")]
pub struct IndicatorClass {}
