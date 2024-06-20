use leptos::*;
use radix_leptos_focus_scope::*;

#[component]
pub fn Basic() -> impl IntoView {
    let (trapped, set_trapped) = create_signal(false);
    let (has_destroy_button, set_has_destroy_button) = create_signal(true);

    view! {
        <div>
            <button type="button" on:click=move |_| set_trapped.set(true)>
                Trap
            </button>
            <input /> <input />
        </div>
        <Show when=move || trapped.get()>
            <FocusScope as_child=true r#loop={trapped} trapped={trapped}>
                <form
                    style:display="inline-flex"
                    style:flex-direction="column"
                    style:gap="20px"
                    style:padding="20px"
                    style:margin="50px"
                    style:max-width="500px"
                    style:border="2px solid"
                >
                    <input type="text" placeholder="First name" />
                    <input type="text" placeholder="Last name" />
                    <input type="number" placeholder="Age" />
                    <Show when=move || has_destroy_button.get()>
                        <div>
                            <button type="button" on:click=move |_| set_has_destroy_button.set(false)>
                                Destroy me
                            </button>
                        </div>
                    </Show>
                    <button type="button" on:click=move |_| set_trapped.set(false)>
                        Close
                    </button>
                </form>
            </FocusScope>
        </Show>
        <div>
            <input /> <input />
        </div>
    }
}

#[component]
pub fn Multiple() -> impl IntoView {
    let (trapped1, set_trapped1) = create_signal(false);
    let (trapped2, set_trapped2) = create_signal(false);

    view! {
        <div
            style:display="inline-flex"
            style:flex-direction="column"
            style:gap="10px"
        >
            <div>
                <button type="button" on:click=move |_| set_trapped1.set(true)>
                    Trap 1
                </button>
            </div>
            <Show when=move || trapped1.get()>
                <FocusScope as_child=true r#loop={trapped1} trapped={trapped1}>
                    <form
                        style:display="inline-flex"
                        style:flex-direction="column"
                        style:gap="20px"
                        style:padding="20px"
                        style:margin="50px"
                        style:max-width="500px"
                        style:border="2px solid"
                    >
                        <input type="text" placeholder="First name" />
                        <input type="text" placeholder="Last name" />
                        <input type="number" placeholder="Age" />
                        <button type="button" on:click=move |_| set_trapped1.set(false)>
                            Close
                        </button>
                    </form>
                </FocusScope>
            </Show>
            <div>
                <button type="button" on:click=move |_| set_trapped2.set(true)>
                    Trap 2
                </button>
            </div>
            <Show when=move || trapped2.get()>
                <FocusScope as_child=true r#loop={trapped2} trapped={trapped2}>
                    <form
                        style:display="inline-flex"
                        style:flex-direction="column"
                        style:gap="20px"
                        style:padding="20px"
                        style:margin="50px"
                        style:max-width="500px"
                        style:border="2px solid"
                    >
                        <input type="text" placeholder="First name" />
                        <input type="text" placeholder="Last name" />
                        <input type="number" placeholder="Age" />
                        <button type="button" on:click=move |_| set_trapped2.set(false)>
                            Close
                        </button>
                    </form>
                </FocusScope>
            </Show>
            <div>
                <input />
            </div>
        </div>
    }
}
