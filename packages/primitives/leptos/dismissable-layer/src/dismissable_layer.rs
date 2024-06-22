use leptos::ev::{CustomEvent, FocusEvent, PointerEvent};

pub struct PointerDownOutsideEventDetail {
    pub original_event: PointerEvent,
}

pub type PointerDownOutsideEvent = CustomEvent;

pub struct FocusOutsideEventDetail {
    pub original_event: FocusEvent,
}

pub type FocusOutsideEvent = CustomEvent;

pub enum InteractOutsideEvent {
    PointerDownOutside(PointerDownOutsideEvent),
    FocusOutside(FocusOutsideEvent),
}
