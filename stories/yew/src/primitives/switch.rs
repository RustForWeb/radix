use radix_yew_label::*;
use radix_yew_switch::*;
use tailwind_fuse::*;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    let label_class = use_memo((), |_| LabelClass::default().to_class());
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let thumb_class = use_memo((), |_| ThumbClass::default().to_class());

    html! {
        <>
            <p>{"This switch is nested inside a label. The state is uncontrolled."}</p>
            <Label class={(*label_class).clone()}>
                {"This is the label "}
                <Switch class={(*root_class).clone()}>
                    <SwitchThumb class={(*thumb_class).clone()} />
                </Switch>
            </Label>
        </>
    }
}

#[function_component]
pub fn Controlled() -> Html {
    let label_class = use_memo((), |_| LabelClass::default().to_class());
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let thumb_class = use_memo((), |_| ThumbClass::default().to_class());

    let checked = use_state_eq(|| true);

    let handle_checked_change = use_callback(checked.clone(), |value, checked| checked.set(value));

    html! {
        <>
            <p>{"This switch is placed adjacent to its label. The state is controlled."}</p>
            <Label class={(*label_class).clone()} r#for="randBox">{"This is the label"}</Label>{" "}
            <Switch
                class={(*root_class).clone()}
                checked={*checked}
                on_checked_change={handle_checked_change}
            >
                <SwitchThumb class={(*thumb_class).clone()} />
            </Switch>
        </>
    }
}

#[function_component]
pub fn WithinForm() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let thumb_class = use_memo((), |_| ThumbClass::default().to_class());

    #[derive(PartialEq)]
    struct Data {
        optional: bool,
        required: bool,
        stopprop: bool,
    }

    let data = use_state_eq(|| Data {
        optional: false,
        required: false,
        stopprop: false,
    });
    let checked = use_state_eq(|| false);

    let handle_change = use_callback(data.clone(), |event: Event, data| {
        // This event does not exist in the DOM, only in React.
        // To make this story functional, on_checked_change event handlers were used instead.

        let input = event.target_unchecked_into::<web_sys::HtmlInputElement>();

        match input.name().as_str() {
            "optional" => data.set(Data {
                optional: input.checked(),
                ..**data
            }),
            "required" => data.set(Data {
                required: input.checked(),
                ..**data
            }),
            "stopprop" => data.set(Data {
                stopprop: input.checked(),
                ..**data
            }),
            _ => unreachable!("No other inputs exist."),
        }
    });

    let handle_change_optional =
        use_callback((checked.clone(), data.clone()), |value, (checked, data)| {
            checked.set(value);
            data.set(Data {
                optional: value,
                ..**data
            })
        });
    let handle_change_required = use_callback(data.clone(), |value, data| {
        data.set(Data {
            required: value,
            ..**data
        })
    });

    html! {
        <form
            onsubmit={Callback::from(|event: SubmitEvent| event.prevent_default())}
            onchange={handle_change}
        >
            <fieldset>
                <legend>{"optional checked: "}{data.optional}</legend>
                <label>
                    <Switch
                        class={(*root_class).clone()}
                        name="optional"
                        checked={*checked}
                        on_checked_change={handle_change_optional}
                    >
                        <SwitchThumb class={(*thumb_class).clone()} />
                    </Switch>{' '}
                    {"with label"}
                </label>
            </fieldset>

            <br />
            <br />

            <fieldset>
                <legend>{"required checked: "}{format!("{}", data.required)}</legend>
                <Switch
                    class={(*root_class).clone()}
                    name="required"
                    required=true
                    on_checked_change={handle_change_required}
                >
                    <SwitchThumb class={(*thumb_class).clone()} />
                </Switch>
            </fieldset>

            <br />
            <br />

            <fieldset>
                <legend>{"stop propagation checked: "}{format!("{}", data.stopprop)}</legend>
                <Switch
                    class={(*root_class).clone()}
                    name="stopprop"
                    on_click={Callback::from(|event: MouseEvent| event.stop_propagation())}
                >
                    <SwitchThumb class={(*thumb_class).clone()} />
                </Switch>
            </fieldset>

            <br />
            <br />

            <button>{"Submit"}</button>
        </form>
    }
}

#[function_component]
pub fn Chromatic() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let thumb_class = use_memo((), |_| ThumbClass::default().to_class());
    let root_attr_class = use_memo((), |_| RootAttrClass::default().to_class());
    let thumb_attr_class = use_memo((), |_| ThumbAttrClass::default().to_class());

    html! {
        <>
            <h1>{"Uncontrolled"}</h1>
            <h2>{"Off"}</h2>
            <Switch class={(*root_class).clone()}>
                <SwitchThumb class={(*thumb_class).clone()} />
            </Switch>

            <h2>{"On"}</h2>
            <Switch default_checked=true class={(*root_class).clone()}>
                <SwitchThumb class={(*thumb_class).clone()} />
            </Switch>

            <h1>{"Controlled"}</h1>
            <h2>{"Off"}</h2>
            <Switch checked=false class={(*root_class).clone()}>
                <SwitchThumb class={(*thumb_class).clone()} />
            </Switch>

            <h2>{"On"}</h2>
            <Switch checked=true class={(*root_class).clone()}>
                <SwitchThumb class={(*thumb_class).clone()} />
            </Switch>

            <h1>{"Disabled"}</h1>
            <Switch disabled=true class={(*root_class).clone()}>
                <SwitchThumb class={(*thumb_class).clone()} />
            </Switch>

            <h1>{"State attributes"}</h1>
            <h2>{"Unchecked"}</h2>
            <Switch class={(*root_attr_class).clone()}>
                <SwitchThumb class={(*thumb_attr_class).clone()} />
            </Switch>

            <h2>{"Checked"}</h2>
            <Switch default_checked=true class={(*root_attr_class).clone()}>
                <SwitchThumb class={(*thumb_attr_class).clone()} />
            </Switch>

            <h2>{"Disabled"}</h2>
            <Switch default_checked=true disabled=true class={(*root_attr_class).clone()}>
                <SwitchThumb class={(*thumb_attr_class).clone()} />
            </Switch>
        </>
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
