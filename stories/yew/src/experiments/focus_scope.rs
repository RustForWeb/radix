use yew::prelude::*;

#[function_component]
pub fn Experiments() -> Html {
    let on_key_down = use_callback((), |event: KeyboardEvent, ()| {
        log::info!("key down {:?}", event.current_target());
    });

    html! {
        <form onkeydown={on_key_down}>
            <input />
            <input />
            <input />
        </form>
    }
}
