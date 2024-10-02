use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct ListProps {
    #[prop_or_default]
    children: Html,
}

#[function_component]
fn List(props: &ListProps) -> Html {
    html! {
        <ul>
            {props.children.clone()}
        </ul>
    }
}

#[derive(PartialEq, Properties)]
struct ItemProps {
    #[prop_or(false)]
    disabled: bool,
    #[prop_or_default]
    children: Html,
}

#[function_component]
fn Item(props: &ItemProps) -> Html {
    html! {
        <li style={format!("color: {};", match props.disabled {
            true => "gray",
            false => "black"
        })}>
            {props.children.clone()}
        </li>
    }
}

#[derive(PartialEq, Properties)]
struct LogItemsProps<T: PartialEq> {
    value: T,
}

#[function_component]
fn LogItems<T: PartialEq>(_props: &LogItemsProps<T>) -> Html {
    use_effect(|| {
        log::info!("log items");
    });

    Html::default()
}

#[function_component]
pub fn Experiments() -> Html {
    let visible = use_state_eq(|| false);
    let disabled = use_state_eq(|| false);

    let on_click_visible = use_callback(visible.clone(), |_, visible| {
        visible.set(!**visible);
    });
    let on_click_disabled = use_callback(disabled.clone(), |_, disabled| {
        disabled.set(!**disabled);
    });

    html! {
        <>
            <button onclick={on_click_visible}>
                {match *visible {
                    true => "Hide",
                    false => "Show"
                }}
            </button>
            <button onclick={on_click_disabled}>
                {match *disabled {
                    true => "Enable",
                    false => "Disable"
                }}
            </button>

            <List>
                <Item>{"Item 1"}</Item>
                <Item disabled={*disabled}>{"Item 2"}</Item>
                <Item>{"Item 3"}</Item>
                if *visible {
                    <Item>{"Item 4"}</Item>
                }
                <LogItems<(bool, bool)> value={(*visible, *disabled)} />
            </List>
        </>
    }
}
