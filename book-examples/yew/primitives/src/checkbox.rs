use radix_yew_checkbox::*;
use radix_yew_icons::CheckIcon;
use yew::prelude::*;

#[function_component]
pub fn CheckboxDemo() -> Html {
    html! {
        <form>
            <div class="flex items-center">
                <Checkbox
                    id="c1"
                    class="shadow-blackA4 hover:bg-violet3 flex h-[25px] w-[25px] appearance-none items-center justify-center rounded-[4px] bg-white shadow-[0_2px_10px] outline-none focus:shadow-[0_0_0_2px_black]"
                    default_checked={CheckedState::True}
                >
                    <CheckboxIndicator class="text-violet11">
                        <CheckIcon />
                    </CheckboxIndicator>
                </Checkbox>
                <label class="pl-[15px] text-[15px] leading-none text-white" for="c1">
                    {"Accept terms and conditions."}
                </label>
            </div>
        </form>
    }
}
