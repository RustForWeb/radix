use leptos::*;
use radix_leptos_switch::*;

#[component]
pub fn SwitchDemo() -> impl IntoView {
    view! {
        <form>
            <div attr:class="flex items-center">
                <label attr:class="text-white text-[15px] leading-none pr-[15px]" attr:for="airplane-mode">
                    Airplane mode
                </label>
                <Switch
                    attr:id="airplane-mode"
                    attr:class="w-[42px] h-[25px] bg-blackA6 rounded-full relative shadow-[0_2px_10px] shadow-blackA4 focus:shadow-[0_0_0_2px] focus:shadow-black data-[state=checked]:bg-black outline-none cursor-default"
                    // style={{ '-webkit-tap-highlight-color': 'rgba(0, 0, 0, 0)' }}
                >
                    <SwitchThumb attr:class="block w-[21px] h-[21px] bg-white rounded-full shadow-[0_2px_2px] shadow-blackA4 transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]" />
                </Switch>
            </div>
        </form>
    }
}
