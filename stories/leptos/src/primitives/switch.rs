use leptos::{ev::Event, *};
use radix_leptos_label::*;
use radix_leptos_switch::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let label_class = Memo::new(move |_| LabelClass::default().to_class());
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let thumb_class = Memo::new(move |_| ThumbClass::default().to_class());

    view! {
        <p>This switch is nested inside a label. The state is uncontrolled.</p>
        <Label attr:class=label_class>
            This is the label{' '}
            <Switch attr:class=root_class>
                <SwitchThumb attr:class=thumb_class />
            </Switch>
        </Label>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let label_class = Memo::new(move |_| LabelClass::default().to_class());
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let thumb_class = Memo::new(move |_| ThumbClass::default().to_class());

    let (checked, set_checked) = create_signal(true);

    view! {
        <p>This switch is placed adjacent to its label. The state is controlled.</p>
        <Label attr:for="randBox" attr:class=label_class>This is the label</Label>{' '}
        <Switch
            attr:id="randBox"
            attr:class=root_class
            checked=checked
            on_checked_change=move |checked| set_checked.set(checked)
        >
            <SwitchThumb attr:class=thumb_class />
        </Switch>
    }
}

#[component]
pub fn WithinForm() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let thumb_class = Memo::new(move |_| ThumbClass::default().to_class());

    struct Data {
        optional: bool,
        required: bool,
        stopprop: bool,
    }

    let (data, set_data) = create_signal(Data {
        optional: false,
        required: false,
        stopprop: false,
    });
    let (checked, set_checked) = create_signal(false);

    view! {
        <form
            on:submit=move |event| event.prevent_default()
            on:change=move |event: Event| {
                // This event does not exist in the DOM, only in React.
                // To make this story functional, on_checked_change event handlers were used instead.

                let input: web_sys::HtmlInputElement = event_target(&event);

                match input.name().as_str() {
                    "optional" => set_data.update(|data| {
                        data.optional = input.checked();
                    }),
                    "required" => set_data.update(|data| {
                        data.required = input.checked();
                    }),
                    "stopprop" => set_data.update(|data| {
                        data.stopprop = input.checked();
                    }),
                    _ => unreachable!("No other inputs exist."),
                }
            }
        >
            <fieldset>
                <legend>optional checked: {move || format!("{}", data.with(|data| data.optional))}</legend>
                <label>
                    <Switch
                        attr:class=root_class
                        name="optional"
                        checked=checked
                        on_checked_change=move |checked| {
                            set_checked.set(checked);
                            set_data.update(|data| {
                                data.optional = checked;
                            })
                        }
                    >
                        <SwitchThumb attr:class=thumb_class />
                    </Switch>{' '}
                    with label
                </label>
            </fieldset>

            <br />
            <br />

            <fieldset>
                <legend>required checked: {move || format!("{}", data.with(|data| data.required))}</legend>
                <Switch
                    attr:class=root_class
                    name="required"
                    required=true
                    on_checked_change=move |checked| {
                        set_data.update(|data| {
                            data.required = checked;
                        });
                    }
                >
                    <SwitchThumb attr:class=thumb_class />
                </Switch>
            </fieldset>


            <br />
            <br />

            <fieldset>
                <legend>stop propagation checked: {move || format!("{}", data.with(|data| data.stopprop))}</legend>
                <Switch
                    attr:class=root_class
                    name="stopprop"
                    on:click=move |event| event.stop_propagation()
                >
                    <SwitchThumb attr:class=thumb_class />
                </Switch>
            </fieldset>

            <br />
            <br />

            <button>Submit</button>
        </form>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let thumb_class = Memo::new(move |_| ThumbClass::default().to_class());
    let root_attr_class = Memo::new(move |_| RootAttrClass::default().to_class());
    let thumb_attr_class = Memo::new(move |_| ThumbAttrClass::default().to_class());

    view! {
        <h1>Uncontrolled</h1>
        <h2>Off</h2>
        <Switch attr:class=root_class>
            <SwitchThumb attr:class=thumb_class />
        </Switch>

        <h2>On</h2>
        <Switch attr:class=root_class default_checked=true>
            <SwitchThumb attr:class=thumb_class />
        </Switch>

        <h1>Controlled</h1>
        <h2>Off</h2>
        <Switch attr:class=root_class checked=false>
            <SwitchThumb attr:class=thumb_class />
        </Switch>

        <h2>On</h2>
        <Switch attr:class=root_class checked=true>
            <SwitchThumb attr:class=thumb_class />
        </Switch>

        <h1>Disabled</h1>
        <Switch attr:class=root_class disabled=true>
            <SwitchThumb attr:class=thumb_class />
        </Switch>

        <h1>State attributes</h1>
        <h2>Unchecked</h2>
        <Switch attr:class=root_attr_class>
            <SwitchThumb attr:class=thumb_attr_class />
        </Switch>

        <h2>Checked</h2>
        <Switch attr:class=root_attr_class default_checked=true>
            <SwitchThumb attr:class=thumb_attr_class />
        </Switch>

        <h2>Disabled</h2>
        <Switch attr:class=root_attr_class default_checked=true disabled=true>
            <SwitchThumb attr:class=thumb_attr_class />
        </Switch>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "inline-block align-middle cursor-default")]
struct LabelClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "align-middle text-left outline-none border-none w-[50px] p-[4px] rounded-[9999px] bg-[#aaa] transition-[background-color] duration-[166ms] ease-[ease-out] box-border leading-[normal] focus:outline-none focus:shadow-[0_0_0_2px_#111] data-[state=checked]:bg-[crimson] data-[state=checked]:border-[crimson] data-[disabled]:opacity-50"
)]
struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-block align-middle w-[20px] h-[20px] bg-[#fff] rounded-[9999px] transition-[transform] duration-[166ms] ease-[ease-out] data-[state=checked]:translate-x-[calc(50px-4px*2-20px)]"
)]
struct ThumbClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] box-border leading-[normal] data-[state=unchecked]:border-[red] data-[state=checked]:border-[green] data-[state=indeterminate]:border-[purple] data-[disabled]:border-dashed disabled:opacity-50"
)]
struct RootAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] data-[state=unchecked]:border-[red] data-[state=checked]:border-[green] data-[state=indeterminate]:border-[purple] data-[disabled]:border-dashed disabled:opacity-50"
)]
struct ThumbAttrClass {}
