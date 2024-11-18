use radix_yew_checkbox::*;
use radix_yew_label::*;
use tailwind_fuse::*;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    let label_class = use_memo((), |_| LabelClass::default().to_class());
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let indicator_class = use_memo((), |_| IndicatorClass::default().to_class());

    html! {
        <>
            <p>{"This checkbox is nested inside a label. The state is uncontrolled."}</p>

            <h1>{"Custom label"}</h1>
            <Label class={(*label_class).clone()}>
                {"Label "}
                <Checkbox class={(*root_class).clone()}>
                    <CheckboxIndicator class={(*indicator_class).clone()} />
                </Checkbox>
            </Label>

            <br />
            <br />

            <h1>{"Native label"}</h1>
            <label>
                {"Label "}
                <Checkbox class={(*root_class).clone()}>
                    <CheckboxIndicator class={(*indicator_class).clone()} />
                </Checkbox>
            </label>

            <h1>{"Native label + native checkbox"}</h1>
            <label>
                {"Label "}<input type="checkbox" />
            </label>

            <h1>{"Custom label + for"}</h1>
            <Label r#for="one">{"Label"}</Label>
            <Checkbox id="one" class={(*root_class).clone()}>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <br />
            <br />

            <h1>{"Native label + for"}</h1>
            <label for="two">{"Label"}</label>
            <Checkbox id="two" class={(*root_class).clone()}>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <h1>{"Native label + native checkbox"}</h1>
            <label for="three">{"Label"}</label>
            <input id="three" type="checkbox" />
        </>
    }
}

#[function_component]
pub fn Controlled() -> Html {
    let label_class = use_memo((), |_| LabelClass::default().to_class());
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let indicator_class = use_memo((), |_| IndicatorClass::default().to_class());

    let checked = use_state_eq(|| CheckedState::True);

    let handle_checked_change = use_callback(checked.clone(), |value, checked| {
        checked.set(value);
    });

    html! {
        <>
            <p>{"This checkbox is placed adjacent to its label. The state is controlled."}</p>
            <Label r#for="randBox" class={(*label_class).clone()}>{"Label"}</Label>{' '}
            <Checkbox
                id="randBox"
                class={(*root_class).clone()}
                checked={*checked}
                on_checked_change={handle_checked_change}
            >
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>
        </>
    }
}

#[function_component]
pub fn Indeterminate() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let indicator_class = use_memo((), |_| IndicatorClass::default().to_class());

    let checked = use_state_eq(|| CheckedState::Indeterminate);

    let handle_checked_change = use_callback(checked.clone(), |value, checked| {
        checked.set(value);
    });
    let handle_click = use_callback(checked.clone(), |_, checked| {
        checked.set(match **checked {
            CheckedState::Indeterminate => CheckedState::False,
            _ => CheckedState::Indeterminate,
        })
    });

    html! {
        <>
            <p>
                <Checkbox
                    class={(*root_class).clone()}
                    checked={*checked}
                    on_checked_change={handle_checked_change}
                >
                    <CheckboxIndicator class={(*indicator_class).clone()} />
                </Checkbox>
            </p>

            <button type="button" onclick={handle_click}>
                {"Toggle indeterminate"}
            </button>
        </>
    }
}

#[function_component]
pub fn WithinForm() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let indicator_class = use_memo((), |_| IndicatorClass::default().to_class());

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
    let checked = use_state_eq(|| CheckedState::Indeterminate);

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
                optional: value == CheckedState::True,
                ..**data
            })
        });
    let handle_change_required = use_callback(data.clone(), |value, data| {
        data.set(Data {
            required: value == CheckedState::True,
            ..**data
        })
    });

    let handle_click = use_callback(checked.clone(), |_, checked| {
        checked.set(match **checked {
            CheckedState::Indeterminate => CheckedState::False,
            _ => CheckedState::Indeterminate,
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
                    <Checkbox
                        class={(*root_class).clone()}
                        name="optional"
                        checked={*checked}
                        on_checked_change={handle_change_optional}
                    >
                        <CheckboxIndicator class={(*indicator_class).clone()} />
                    </Checkbox>{' '}
                    {"with label"}
                </label>
                <br />
                <br />

                <button type="button" onclick={handle_click}>
                    {"Toggle indeterminate"}
                </button>
            </fieldset>

            <br />
            <br />

            <fieldset>
                <legend>{"required checked: "}{format!("{}", data.required)}</legend>
                <Checkbox
                    class={(*root_class).clone()}
                    name="required"
                    required=true
                    on_checked_change={handle_change_required}
                >
                    <CheckboxIndicator class={(*indicator_class).clone()} />
                </Checkbox>
            </fieldset>


            <br />
            <br />

            <fieldset>
                <legend>{"stop propagation checked: "}{format!("{}", data.stopprop)}</legend>
                <Checkbox
                    class={(*root_class).clone()}
                    name="stopprop"
                    on_click={Callback::from(|event: MouseEvent| event.stop_propagation())}
                >
                    <CheckboxIndicator class={(*indicator_class).clone()} />
                </Checkbox>
            </fieldset>

            <br />
            <br />

            <button type="reset">{"Reset"}</button>
            <button>{"Submit"}</button>
        </form>
    }
}

#[function_component]
pub fn Animated() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let animated_indicator_class = use_memo((), |_| AnimatedIndicatorClass::default().to_class());

    let checked = use_state_eq(|| CheckedState::Indeterminate);

    let handle_checked_change = use_callback(checked.clone(), |value, checked| {
        checked.set(value);
    });
    let handle_click = use_callback(checked.clone(), |_, checked| {
        checked.set(match **checked {
            CheckedState::Indeterminate => CheckedState::False,
            _ => CheckedState::Indeterminate,
        })
    });

    html! {
        <>
            <p>
                <Checkbox
                    class={(*root_class).clone()}
                    checked={*checked}
                    on_checked_change={handle_checked_change}
                >
                    <CheckboxIndicator class={(*animated_indicator_class).clone()} />
                </Checkbox>
            </p>

            <button type="button" onclick={handle_click}>
                {"Toggle indeterminate"}
            </button>
        </>
    }
}

#[function_component]
pub fn Chromatic() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());
    let indicator_class = use_memo((), |_| IndicatorClass::default().to_class());
    let root_attr_class = use_memo((), |_| RootAttrClass::default().to_class());
    let indicator_attr_class = use_memo((), |_| IndicatorAttrClass::default().to_class());

    html! {
        <>
            <h1>{"Uncontrolled"}</h1>
            <h2>{"Unchecked"}</h2>
            <Checkbox class={(*root_class).clone()}>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <h2>{"Checked"}</h2>
            <Checkbox class={(*root_class).clone()} default_checked={CheckedState::True}>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <h1>{"Controlled"}</h1>
            <h2>{"Unchecked"}</h2>
            <Checkbox class={(*root_class).clone()} checked={CheckedState::False}>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <h2>{"Checked"}</h2>
            <Checkbox class={(*root_class).clone()} checked={CheckedState::True}>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <h1>{"Indeterminate"}</h1>
            <Checkbox class={(*root_class).clone()} checked={CheckedState::Indeterminate}>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <h1>{"Disabled"}</h1>
            <Checkbox class={(*root_class).clone()} default_checked={CheckedState::True} disabled=true>
                <CheckboxIndicator class={(*indicator_class).clone()} />
            </Checkbox>

            <h1>{"Force mounted indicator"}</h1>
            <Checkbox class={(*root_class).clone()}>
                <CheckboxIndicator class={(*indicator_class).clone()} force_mount=true style={[("height", "20px")]} />
            </Checkbox>

            <h1>{"State attributes"}</h1>
            <h2>{"Unchecked"}</h2>
            <Checkbox class={(*root_attr_class).clone()}>
                <CheckboxIndicator class={(*indicator_attr_class).clone()} />
            </Checkbox>

            <h2>{"Checked"}</h2>
            <Checkbox class={(*root_attr_class).clone()} default_checked={CheckedState::True}>
                <CheckboxIndicator class={(*indicator_attr_class).clone()} />
            </Checkbox>

            <h2>{"Indeterminate"}</h2>
            <Checkbox class={(*root_attr_class).clone()} checked={CheckedState::Indeterminate}>
                <CheckboxIndicator class={(*indicator_attr_class).clone()} />
            </Checkbox>

            <h2>{"Disabled"}</h2>
            <Checkbox class={(*root_attr_class).clone()} default_checked={CheckedState::True} disabled=true>
                <CheckboxIndicator class={(*indicator_attr_class).clone()} />
            </Checkbox>

            <h2>{"Force mounted indicator"}</h2>
            <Checkbox class={(*root_attr_class).clone()}>
                <CheckboxIndicator class={(*indicator_attr_class).clone()} force_mount=true style={[("height", "20px")]} />
            </Checkbox>
        </>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "inline-block align-middle cursor-default")]
struct LabelClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "align-middle border-[1px] border-solid border-[#aaa] w-[30px] h-[30px] p-[4px] box-border leading-[normal] focus:outline-none focus:border-[crimson] focus:shadow-[0_0_0_1px_crimson] data-[disabled]:opacity-30"
)]
struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[crimson] block w-[20px] h-[4px] data-[state=checked]:h-[20px] data-[state=unchecked]:h-[20px]"
)]
struct IndicatorClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[crimson] block w-[20px] h-[4px] data-[state=checked]:h-[20px] data-[state=unchecked]:h-[20px] transition-[height] duration-[300ms] data-[state=checked]:animate-[checkboxFadeIn_1000ms_ease-out] data-[state=unchecked]:animate-[checkboxFadeOut_1000ms_ease-in]"
)]
struct AnimatedIndicatorClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] box-border leading-[normal] data-[state=unchecked]:border-[red] data-[state=checked]:border-[green] data-[state=indeterminate]:border-[purple] data-[disabled]:border-dashed disabled:opacity-50"
)]
struct RootAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] leading-[normal] data-[state=unchecked]:border-[red] data-[state=checked]:border-[green] data-[state=indeterminate]:border-[purple] data-[disabled]:border-dashed disabled:opacity-50"
)]
struct IndicatorAttrClass {}
