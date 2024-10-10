use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SlotProps<T: Clone + PartialEq> {
    #[prop_or_default]
    as_child: Option<Callback<T, Html>>,
    #[prop_or_default]
    as_child_props: Option<T>,
    #[prop_or_default]
    children: Html,
}

#[function_component]
pub fn Slot<T: Clone + PartialEq = ()>(props: &SlotProps<T>) -> Html {
    html! {
        if let Some(as_child) = props.as_child.as_ref() {
            {as_child.emit(props.as_child_props.as_ref().expect("`as_child_props` is required when using `as_child`.").clone())}
        } else {
            {props.children.clone()}
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ButtonAsChildProps {
    disabled: bool,
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    as_child: Option<Callback<ButtonAsChildProps, Html>>,
    #[prop_or_default]
    children: Html,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let as_child_props = ButtonAsChildProps { disabled: true };

    html! {
        <Slot<ButtonAsChildProps>
            as_child={props.as_child.clone()}
            as_child_props={as_child_props}
        >
            {props.children.clone()}
        </Slot<ButtonAsChildProps>>
    }
}

#[function_component]
pub fn Experiments() -> Html {
    html! {
        <>
            <Slot>
                <button>{"Click me"}</button>
            </Slot>

            <Slot
                as_child={Callback::from(|_| html! {
                    <button>{"Click me"}</button>
                })}
                as_child_props={}
            />

            <Button>{"Click me"}</Button>

            <Button
                as_child={Callback::from(|ButtonAsChildProps { disabled } | html! {
                    <a href="#" style={disabled.then_some("pointer-events: none;")}>{"Click me"}</a>
                })}
            />
        </>
    }
}
