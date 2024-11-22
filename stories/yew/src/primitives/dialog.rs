use radix_yew_dialog::*;
use tailwind_fuse::*;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    let trigger_class = use_memo((), |_| TriggerClass::default().to_class());
    let overlay_class = use_memo((), |_| OverlayClass::default().to_class());
    let content_default_class = use_memo((), |_| ContentDefaultClass::default().to_class());
    let close_class = use_memo((), |_| CloseClass::default().to_class());

    html! {
        <Dialog>
            <DialogTrigger class={(*trigger_class).clone()}>{"open"}</DialogTrigger>
            <DialogPortal>
                <DialogOverlay class={(*overlay_class).clone()} />
                <DialogContent class={(*content_default_class).clone()}>
                    <DialogTitle>{"Booking info"}</DialogTitle>
                    <DialogDescription>{"Please enter the info for your booking below."}</DialogDescription>
                    <DialogClose class={(*close_class).clone()}>{"close"}</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[function_component]
pub fn NonModal() -> Html {
    html! {}
}

#[function_component]
pub fn Controlled() -> Html {
    html! {}
}

#[function_component]
pub fn FocusTrap() -> Html {
    html! {}
}

#[function_component]
pub fn CustomFocus() -> Html {
    html! {}
}

#[function_component]
pub fn NoEscapeDismiss() -> Html {
    html! {}
}

#[function_component]
pub fn NoPointerDownOutsideDismiss() -> Html {
    html! {}
}

#[function_component]
pub fn WithPortalContainer() -> Html {
    html! {}
}

#[function_component]
pub fn Animated() -> Html {
    html! {}
}

#[function_component]
pub fn ForcedMount() -> Html {
    html! {}
}

#[function_component]
pub fn InnerScrollable() -> Html {
    html! {}
}

#[function_component]
pub fn OuterScrollable() -> Html {
    html! {}
}

#[function_component]
pub fn Chromatic() -> Html {
    html! {}
}

#[function_component]
pub fn Cypress() -> Html {
    html! {}
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct TriggerClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "fixed top-0 right-0 bottom-0 left-0 bg-[rgba(0,0,0,0.2)]")]
struct OverlayClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "fixed top-0 right-0 bottom-0 left-0 bg-[rgba(0,0,0,0.2)] overflow-auto flex items-start justify-center"
)]
struct ScrollableOverlayClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "fixed min-w-[300px] min-h-[150px] p-[50px] rounded-[10px] bg-[white] shadow-[0_2px_10px_rgba(0,0,0,0.12)] top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
)]
struct ContentDefaultClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "fixed min-w-[300px] min-h-[150px] p-[50px] rounded-[10px] bg-[white] shadow-[0_2px_10px_rgba(0,0,0,0.12)] top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 overflow-auto max-h-[300px]"
)]
struct ContentScrollableClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "min-w-[300px] min-h-[150px] p-[50px] rounded-[10px] bg-[white] shadow-[0_2px_10px_rgba(0,0,0,0.12)] mt-[50px] mb-[50px]"
)]
struct ContentInScrollableOverlayClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "fixed top-0 min-w-[300px] p-[50px] rounded-[10px] bg-[white] shadow-[0_2px_10px_rgba(0,0,0,0.12)] right-0 min-h-[100vh] rounded-tr-none rounded-br-none"
)]
struct ContentSheetClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct CloseClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct AnimatedOverlayClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct AnimatedContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct ChromaticContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct TriggerAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct OverlayAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct ContentAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "")]
struct CloseAttrClass {}
