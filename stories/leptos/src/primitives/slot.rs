use leptos::*;
use radix_leptos_slot::*;

#[component]
pub fn WithoutSlottable() -> impl IntoView {
    view! {
        <SlotWithoutSlottable>
            <b data-slot-element>hello</b>
        </SlotWithoutSlottable>
    }
}

#[component]
pub fn WithSlottable() -> impl IntoView {
    view! {
        <SlotWithSlottable>
            <b data-slot-element>hello</b>
        </SlotWithSlottable>
    }
}

#[component]
fn SlotWithoutSlottable(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Slot attrs=attrs>
            {children()}
        </Slot>
    }
}

#[component]
fn SlotWithSlottable(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Slot attrs=attrs>
            <Slottable>{children.with_value(|children| children())}</Slottable>
            <span>world</span>
        </Slot>
    }
}
