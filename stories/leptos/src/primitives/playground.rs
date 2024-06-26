use leptos::{ev::Event, *};

#[component]
pub fn Parent(#[prop(into, optional)] on_something: Option<Callback<Event>>) -> impl IntoView {
    view! {
        <Child on_something=on_something />
    }
}

#[component]
pub fn Child(
    #[prop(into, optional)] on_something: Option<Option<Callback<Event>>>,
) -> impl IntoView {
    view! {
        <button on:click=move |event| {
            if let Some(on_something) = on_something.flatten() {
                on_something.call(event.into());
            }
        } />
    }
}

#[component]
pub fn Playground() -> impl IntoView {
    view! {
        <Parent />
        <Parent on_something=move |_| println!("parent") />

        <Child />
        <Child on_something=Some(Callback::new(move |_| println!("parent"))) />
    }
}
