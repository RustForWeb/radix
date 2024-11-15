use leptos::*;
use radix_leptos_toggle::*;
use tailwind_fuse::*;

#[component]
pub fn Controlled() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());

    let (pressed, set_pressed) = create_signal(true);

    view! {
        <Toggle attr:class=root_class on_pressed_change=move |pressed| set_pressed.set(pressed)>
            {move || match pressed.get() {
                true => "On",
                false => "Off"
            }}
        </Toggle>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let root_attr_class = Memo::new(move |_| RootAttrClass::default().to_class());

    view! {
        <h1>Uncontrolled</h1>
        <h2>Off</h2>
        <Toggle attr:class=root_class>Toggle</Toggle>

        <h2>On</h2>
        <Toggle attr:class=root_class default_pressed=true>Toggle</Toggle>

        <h1>Controlled</h1>
        <h2>Off</h2>
        <Toggle attr:class=root_class pressed=false>Toggle</Toggle>

        <h2>On</h2>
        <Toggle attr:class=root_class pressed=true>Toggle</Toggle>

        <h2>Disabled</h2>
        <Toggle attr:class=root_class disabled=true>Toggle</Toggle>

        <h2>State attributes</h2>
        <Toggle attr:class=root_attr_class>Toggle</Toggle>
        <Toggle attr:class=root_attr_class disabled=true>Toggle</Toggle>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "p-[6px] leading-[1] border-none font-sans font-bold focus:outline-none focus:shadow-[0_0_0_2px_#111] data-[disabled]:opacity-50 data-[state=off]:bg-[red] data-[state=off]:text-[#fff] data-[state=on]:bg-[green] data-[state=on]:text-[#fff]"
)]
struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] disabled:opacity-50 data-[disabled]:border-dashed"
)]
struct RootAttrClass {}
