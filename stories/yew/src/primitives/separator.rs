use radix_yew_separator::*;
use tailwind_fuse::*;
use yew::prelude::*;
use yew_attrs::attrs;

#[function_component]
pub fn Styled() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());

    html! {
        <>
            <h1>{"Horizontal"}</h1>
            <p>{"The following separator is horizontal and has semantic meaning."}</p>
            <Separator orientation={Orientation::Horizontal} attrs={attrs! { class={(*root_class).clone()} }} />
            <p>
                {"The following separator is horizontal and is purely decorative. Assistive technology will
                ignore this element."}
            </p>
            <Separator orientation={Orientation::Horizontal} decorative=true attrs={attrs! { class={(*root_class).clone()} }} />

            <h1>{"Vertical"}</h1>
            <div style="display: flex; align-items: center;">
                <p>{"The following separator is vertical and has semantic meaning."}</p>
                <Separator orientation={Orientation::Vertical} attrs={attrs! { class={(*root_class).clone()} }} />
                <p>
                    {"The following separator is vertical and is purely decorative. Assistive technology will
                    ignore this element."}
                </p>
                <Separator orientation={Orientation::Vertical} decorative=true attrs={attrs! { class={(*root_class).clone()} }} />
            </div>
        </>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "border-none bg-[crimson] data-[orientation=horizontal]:h-[1px] data-[orientation=horizontal]:w-full data-[orientation=horizontal]:m-[20px_0px]  data-[orientation=vertical]:h-[100px] data-[orientation=vertical]:w-[1px] data-[orientation=vertical]:m-[0px_20px]"
)]
pub struct RootClass {}
