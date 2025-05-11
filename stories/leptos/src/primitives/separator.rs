use leptos::prelude::*;
use radix_leptos_separator::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());

    view! {
        <h1>Horizontal</h1>
        <p>The following separator is horizontal and has semantic meaning.</p>
        <Separator attr:class=root_class orientation=Orientation::Horizontal />
        <p>
            The following separator is horizontal and is purely decorative. Assistive technology will
            ignore this element.
        </p>
        <Separator attr:class=root_class orientation=Orientation::Horizontal decorative=true />

        <h1>Vertical</h1>
        <div style:display="flex" style:align-items="center">
            <p>The following separator is vertical and has semantic meaning.</p>
            <Separator attr:class=root_class orientation=Orientation::Vertical />
            <p>
                The following separator is vertical and is purely decorative. Assistive technology will
                ignore this element.
            </p>
            <Separator attr:class=root_class orientation=Orientation::Vertical decorative=true />
        </div>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "border-none bg-[crimson] data-[orientation=horizontal]:h-[1px] data-[orientation=horizontal]:w-full data-[orientation=horizontal]:m-[20px_0px]  data-[orientation=vertical]:h-[100px] data-[orientation=vertical]:w-[1px] data-[orientation=vertical]:m-[0px_20px]"
)]
struct RootClass {}
