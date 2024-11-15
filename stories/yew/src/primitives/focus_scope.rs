use radix_yew_focus_scope::*;
use yew::prelude::*;

#[function_component]
pub fn Basic() -> Html {
    let trapped = use_state(|| false);
    let has_destroy_button = use_state(|| true);

    let handle_click_trap = use_callback(trapped.clone(), |_, trapped| trapped.set(true));
    let handle_click_untrap = use_callback(trapped.clone(), |_, trapped| trapped.set(false));
    let handle_click_destroy = use_callback(has_destroy_button.clone(), |_, has_destroy_button| {
        has_destroy_button.set(false)
    });

    html! {
        <>
            <div>
                <button type="button" onclick={handle_click_trap}>
                    {"Trap"}
                </button>
                <input /> <input />
            </div>
            if *trapped {
                <FocusScope
                    r#loop={*trapped}
                    trapped={*trapped}
                    as_child={Callback::from(move |FocusScopeChildProps { node_ref, tabindex, onkeydown, .. }| html! {
                        <form
                            ref={node_ref}
                            tabindex={tabindex}
                            onkeydown={onkeydown}
                            style="display: inline-flex; flex-direction: column; gap: 20px; padding: 20px; margin: 50px; max-width: 500px; border: 2px solid;"
                        >
                            <input type="text" placeholder="First name" />
                            <input type="text" placeholder="Last name" />
                            <input type="number" placeholder="Age" />
                            if *has_destroy_button {
                                <div>
                                    <button type="button" onclick={handle_click_destroy.clone()}>
                                        {"Destroy me"}
                                    </button>
                                </div>
                            }
                            <button type="button" onclick={handle_click_untrap.clone()}>
                                {"Close"}
                            </button>
                        </form>
                    })}
                />
            }
            <div>
                <input /> <input />
            </div>
        </>
    }
}

#[function_component]
pub fn Multiple() -> Html {
    let trapped1 = use_state(|| false);
    let trapped2 = use_state(|| false);

    let handle_click_trap1 = use_callback(trapped1.clone(), |_, trapped1| trapped1.set(true));
    let handle_click_untrap1 = use_callback(trapped1.clone(), |_, trapped1| trapped1.set(false));
    let handle_click_trap2 = use_callback(trapped2.clone(), |_, trapped2| trapped2.set(true));
    let handle_click_untrap2 = use_callback(trapped2.clone(), |_, trapped2| trapped2.set(false));

    html! {
        <div style="display: inline-flex; flex-direction: column; gap: 10px;">
            <div>
                <button type="button" onclick={handle_click_trap1}>
                    {"Trap 1"}
                </button>
            </div>
            if *trapped1 {
                <FocusScope
                    r#loop={*trapped1}
                    trapped={*trapped1}
                    as_child={Callback::from(move |FocusScopeChildProps { node_ref, tabindex, onkeydown, .. }| html! {
                        <form
                            ref={node_ref}
                            tabindex={tabindex}
                            onkeydown={onkeydown}
                            style="display: inline-flex; flex-direction: column; gap: 20px; padding: 20px; margin: 50px; max-width: 500px; border: 2px solid;"
                        >
                            <input type="text" placeholder="First name" />
                            <input type="text" placeholder="Last name" />
                            <input type="number" placeholder="Age" />
                            <button type="button" onclick={handle_click_untrap1.clone()}>
                                {"Close"}
                            </button>
                        </form>
                    })}
                />
            }
            <div>
                <button type="button" onclick={handle_click_trap2}>
                    {"Trap 2"}
                </button>
            </div>
            if *trapped2 {
                <FocusScope
                    r#loop={*trapped2}
                    trapped={*trapped2}
                    as_child={Callback::from(move |FocusScopeChildProps { node_ref, tabindex, onkeydown, ..}| html! {
                        <form
                            ref={node_ref}
                            tabindex={tabindex}
                            onkeydown={onkeydown}
                            style="display: inline-flex; flex-direction: column; gap: 20px; padding: 20px; margin: 50px; max-width: 500px; border: 2px solid;"
                        >
                            <input type="text" placeholder="First name" />
                            <input type="text" placeholder="Last name" />
                            <input type="number" placeholder="Age" />
                            <button type="button" onclick={handle_click_untrap2.clone()}>
                                {"Close"}
                            </button>
                        </form>
                    })}
                />
            }
            <div>
                <input />
            </div>
        </div>
    }
}

#[function_component]
pub fn WithOptions() -> Html {
    html! {
        // TODO
    }
}
