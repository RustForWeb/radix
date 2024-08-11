use leptos::{ev::Event, *};
use radix_leptos_checkbox::*;
use radix_leptos_label::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let label_class = Memo::new(move |_| LabelClass::default().to_class());
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let indicator_class = Memo::new(move |_| IndicatorClass::default().to_class());

    view! {
        <p>This checkbox is nested inside a label. The state is uncontrolled.</p>

        <h1>Custom label</h1>
        <Label attr:class=label_class>
            Label{' '}
            <Checkbox attr:class=root_class>
                <CheckboxIndicator attr:class=indicator_class />
            </Checkbox>
        </Label>

        <br />
        <br />

        <h1>Native label</h1>
        <label>
            Label{' '}
            <Checkbox attr:class=root_class>
                <CheckboxIndicator attr:class=indicator_class />
            </Checkbox>
        </label>

        <h1>Native label + native checkbox</h1>
        <label>
            Label <input type="checkbox" />
        </label>

        <h1>Custom label + for</h1>
        <Label attr:for="one">Label</Label>
        <Checkbox attr:id="one" attr:class=root_class>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <br />
        <br />

        <h1>Native label + for</h1>
        <label for="two">Label</label>
        <Checkbox attr:id="two" attr:class=root_class>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <h1>Native label + native checkbox</h1>
        <label for="three">Label</label>
        <input id="three" type="checkbox" />
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let label_class = Memo::new(move |_| LabelClass::default().to_class());
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let indicator_class = Memo::new(move |_| IndicatorClass::default().to_class());

    let (checked, set_checked) = create_signal(CheckedState::True);

    view! {
        <p>This checkbox is placed adjacent to its label. The state is controlled.</p>
        <Label attr:for="randBox" attr:class=label_class>Label</Label>{' '}
        <Checkbox
            attr:id="randBox"
            attr:class=root_class
            checked=checked
            on_checked_change=move |checked| set_checked.set(checked)
        >
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>
    }
}

#[component]
pub fn Indeterminate() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let indicator_class = Memo::new(move |_| IndicatorClass::default().to_class());

    let (checked, set_checked) = create_signal(CheckedState::Indeterminate);

    view! {
        <p>
            <Checkbox
                attr:class=root_class
                checked=checked
                on_checked_change=move |checked| set_checked.set(checked)
            >
                <CheckboxIndicator attr:class=indicator_class />
            </Checkbox>
        </p>

        <button
            type="button"
            on:click=move |_| {
                set_checked.update(|checked| {
                    *checked = match checked {
                        CheckedState::Indeterminate => CheckedState::False,
                        _ => CheckedState::Indeterminate,
                    };
                })
            }
        >
            Toggle indeterminate
        </button>
    }
}

#[component]
pub fn WithinForm() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let indicator_class = Memo::new(move |_| IndicatorClass::default().to_class());

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
    let (checked, set_checked) = create_signal(CheckedState::Indeterminate);

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
                    <Checkbox
                        attr:class=root_class
                        name="optional"
                        checked=checked
                        on_checked_change=move |checked| {
                            set_checked.set(checked);
                            set_data.update(|data| {
                                data.optional = checked == CheckedState::True;
                            })
                        }
                    >
                        <CheckboxIndicator attr:class=indicator_class />
                    </Checkbox>{' '}
                    with label
                </label>
                <br />
                <br />

                <button
                    type="button"
                    on:click=move |_| {
                        set_checked.update(|checked| {
                            *checked = match checked {
                                CheckedState::Indeterminate => CheckedState::False,
                                _ => CheckedState::Indeterminate,
                            };
                        })
                    }
                >
                    Toggle indeterminate
                </button>
            </fieldset>

            <br />
            <br />

            <fieldset>
                <legend>required checked: {move || format!("{}", data.with(|data| data.required))}</legend>
                <Checkbox
                    attr:class=root_class
                    name="required"
                    required=true
                    on_checked_change=move |checked| {
                        set_data.update(|data| {
                            data.required = checked == CheckedState::True;
                        });
                    }
                >
                    <CheckboxIndicator attr:class=indicator_class />
                </Checkbox>
            </fieldset>


            <br />
            <br />

            <fieldset>
                <legend>stop propagation checked: {move || format!("{}", data.with(|data| data.stopprop))}</legend>
                <Checkbox
                    attr:class=root_class
                    name="stopprop"
                    on:click=move |event| event.stop_propagation()
                >
                    <CheckboxIndicator attr:class=indicator_class />
                </Checkbox>
            </fieldset>

            <br />
            <br />

            <button type="reset">Reset</button>
            <button>Submit</button>
        </form>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let animated_indicator_class = Memo::new(move |_| AnimatedIndicatorClass::default().to_class());

    let (checked, set_checked) = create_signal(CheckedState::Indeterminate);

    // TODO: fade out doesn't work, might be an issue with Presence component?

    view! {
        <p>
            <Checkbox
                attr:class=root_class
                checked=checked
                on_checked_change=move |checked| set_checked.set(checked)
            >
                <CheckboxIndicator attr:class=animated_indicator_class />
            </Checkbox>
        </p>

        <button
            type="button"
            on:click=move |_| {
                set_checked.update(|checked| {
                    *checked = match checked {
                        CheckedState::Indeterminate => CheckedState::False,
                        _ => CheckedState::Indeterminate,
                    };
                })
            }
        >
            Toggle indeterminate
        </button>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let indicator_class = Memo::new(move |_| IndicatorClass::default().to_class());
    let root_attr_class = Memo::new(move |_| RootAttrClass::default().to_class());
    let indicator_attr_class = Memo::new(move |_| IndicatorAttrClass::default().to_class());

    view! {
        <h1>Uncontrolled</h1>
        <h2>Unchecked</h2>
        <Checkbox attr:class=root_class>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <h2>Checked</h2>
        <Checkbox attr:class=root_class default_checked=CheckedState::True>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <h1>Controlled</h1>
        <h2>Unchecked</h2>
        <Checkbox attr:class=root_class checked=CheckedState::False>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <h2>Checked</h2>
        <Checkbox attr:class=root_class checked=CheckedState::True>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <h1>Indeterminate</h1>
        <Checkbox attr:class=root_class checked=CheckedState::Indeterminate>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <h1>Disabled</h1>
        <Checkbox attr:class=root_class default_checked=CheckedState::True disabled=true>
            <CheckboxIndicator attr:class=indicator_class />
        </Checkbox>

        <h1>Force mounted indicator</h1>
        <Checkbox attr:class=root_class>
            <CheckboxIndicator attr:class=indicator_class force_mount=true />
            {/* style:height="20px" */}
        </Checkbox>

        <h1>State attributes</h1>
        <h2>Unchecked</h2>
        <Checkbox attr:class=root_attr_class>
            <CheckboxIndicator attr:class=indicator_attr_class />
        </Checkbox>

        <h2>Checked</h2>
        <Checkbox attr:class=root_attr_class default_checked=CheckedState::True>
            <CheckboxIndicator attr:class=indicator_attr_class />
        </Checkbox>

        <h2>Indeterminate</h2>
        <Checkbox attr:class=root_attr_class checked=CheckedState::Indeterminate>
            <CheckboxIndicator attr:class=indicator_attr_class />
        </Checkbox>

        <h2>Disabled</h2>
        <Checkbox attr:class=root_attr_class default_checked=CheckedState::True disabled=true>
            <CheckboxIndicator attr:class=indicator_attr_class />
        </Checkbox>

        <h2>Force mounted indicator</h2>
        <Checkbox attr:class=root_class>
            <CheckboxIndicator attr:class=indicator_attr_class force_mount=true />
            {/* style:height="20px" */}
        </Checkbox>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "inline-block align-middle cursor-default")]
pub struct LabelClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "align-middle border-[1px] border-solid border-[#aaa] w-[30px] h-[30px] p-[4px] box-border leading-[normal] focus:outline-none focus:border-[crimson] focus:shadow-[0_0_0_1px_crimson] data-[disabled]:opacity-30"
)]
pub struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[crimson] block w-[20px] h-[4px] data-[state=checked]:h-[20px] data-[state=unchecked]:h-[20px]"
)]
pub struct IndicatorClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[crimson] block w-[20px] h-[4px] data-[state=checked]:h-[20px] data-[state=unchecked]:h-[20px] transition-[height] duration-[300ms] data-[state=checked]:animate-[checkboxFadeIn_1000ms_ease-out] data-[state=unchecked]:animate-[checkboxFadeOut_1000ms_ease-in]"
)]
pub struct AnimatedIndicatorClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] box-border leading-[normal] data-[state=unchecked]:border-[red] data-[state=checked]:border-[green] data-[state=indeterminate]:border-[purple] data-[disabled]:border-dashed disabled:opacity-50"
)]
pub struct RootAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "bg-[rgba(0,0,255,0.3)] border-[2px] border-solid border-[blue] p-[10px] leading-[normal] data-[state=unchecked]:border-[red] data-[state=checked]:border-[green] data-[state=indeterminate]:border-[purple] data-[disabled]:border-dashed disabled:opacity-50"
)]
pub struct IndicatorAttrClass {}
