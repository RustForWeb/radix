use radix_yew_collection::*;
use yew::prelude::*;
use yew_style::Style;

#[function_component]
pub fn Basic() -> Html {
    html! {
        <List>
            <Item>{"Red"}</Item>
            <Item disabled=true>{"Green"}</Item>
            <Item>{"Blue"}</Item>
            <LogItems />
        </List>
    }
}

#[function_component]
pub fn WithElementsInBetween() -> Html {
    html! {
        <List>
            <div style="font-variant: small-caps;">{"Colors"}</div>
            <Item>{"Red"}</Item>
            <Item disabled=true>{"Green"}</Item>
            <Item>{"Blue"}</Item>
            <div style="font-variant: small-caps;">{"Words"}</div>
            <Item>{"Hello"}</Item>
            <Item>{"World"}</Item>
            <LogItems />
        </List>
    }
}
#[function_component]
fn Tomato() -> Html {
    html! {
        <Item style={[("color", "tomato")]}>{"Tomato"}</Item>
    }
}

#[function_component]
pub fn WithWrappedItem() -> Html {
    html! {
        <List>
            <Item>{"Red"}</Item>
            <Item disabled=true>{"Green"}</Item>
            <Item>{"Blue"}</Item>
            <Tomato />
            <LogItems />
        </List>
    }
}

#[function_component]
pub fn WithFragment() -> Html {
    let countries = html! {
        <>
            <Item>{"France"}</Item>
            <Item disabled=true>{"UK"}</Item>
            <Item>{"Spain"}</Item>
        </>
    };

    html! {
        <List>
            {countries}
            <LogItems />
        </List>
    }
}

#[function_component]
pub fn DynamicInsertion() -> Html {
    let has_tomato = use_state_eq(|| false);

    let handle_click = use_callback(has_tomato.clone(), |_, has_tomato| {
        has_tomato.set(!**has_tomato)
    });

    html! {
        <>
            <button onclick={handle_click}>
                {match *has_tomato {
                    true => "Remove",
                    false => "Add"
                }}
            </button>

            <List>
                <WrappedItems has_tomato={*has_tomato} />
                <LogItems<bool> rerender_value={*has_tomato} />
            </List>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct WrappedItemsProps {
    has_tomato: bool,
}

#[function_component]
fn WrappedItems(props: &WrappedItemsProps) -> Html {
    html! {
        <>
            <Item>{"Red"}</Item>
            if props.has_tomato {
                <Tomato />
            }
            <Item disabled=true>{"Green"}</Item>
            <Item>{"Blue"}</Item>
        </>
    }
}

#[function_component]
pub fn WithChangingItem() -> Html {
    let is_disabled = use_state_eq(|| false);

    let handle_click = use_callback(is_disabled.clone(), |_, is_disabled| {
        is_disabled.set(!**is_disabled)
    });

    html! {
        <>
            <button onclick={handle_click}>
                {match *is_disabled {
                    true => "Enable",
                    false => "Disable"
                }}
            </button>

            <List>
                <Item>{"Red"}</Item>
                <Item disabled={*is_disabled}>{"Green"}</Item>
                <Item>{"Blue"}</Item>
                <LogItems<bool> rerender_value={*is_disabled} />
            </List>
        </>
    }
}

#[function_component]
pub fn Nested() -> Html {
    html! {
        <List>
            <Item>{"1"}</Item>
            <Item>
                {"2"}
                <List>
                    <Item>{"2.1"}</Item>
                    <Item>{"2.2"}</Item>
                    <Item>{"2.3"}</Item>
                    <LogItems name="items inside 2" />
                </List>
            </Item>
            <Item>{"3"}</Item>
            <LogItems name="top-level items" />
        </List>
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ItemData {
    #[allow(dead_code)]
    disabled: bool,
}

#[derive(PartialEq, Properties)]
struct ListProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn List(props: &ListProps) -> Html {
    html! {
        <CollectionProvider<ItemData>>
            <CollectionSlot<ItemData>
                as_child={Callback::from({
                    let children = props.children.clone();

                    move |CollectionSlotChildProps { node_ref }| html! {
                        <ul ref={node_ref} style="width: 200px;">
                            {children.clone()}
                        </ul>
                    }
                })}
            />
        </CollectionProvider<ItemData>>
    }
}

#[derive(PartialEq, Properties)]
struct ItemProps {
    #[prop_or(false)]
    disabled: bool,
    #[prop_or_default]
    style: Style,
    #[prop_or_default]
    children: Html,
}

#[function_component]
fn Item(props: &ItemProps) -> Html {
    let item_data = use_memo(props.disabled, |disabled| ItemData {
        disabled: *disabled,
    });

    html! {
        <CollectionItemSlot<ItemData>
            item_data={(*item_data).clone()}
            as_child={Callback::from({
                let disabled = props.disabled;
                let style = props.style.clone();
                let children = props.children.clone();

                move |CollectionItemSlotChildProps { node_ref, data_radix_collection_item }| html! {
                    <li
                        ref={node_ref}
                        data-radix-collection-item={data_radix_collection_item}
                        style={style.clone().with_defaults([
                            ("opacity", disabled.then_some("0.3")),
                        ])}
                    >
                        {children.clone()}
                    </li>
                }
            })}
        />
    }
}

#[derive(PartialEq, Properties)]
struct LogItemsProps<T: Default + PartialEq> {
    #[prop_or("items".to_string())]
    name: String,
    // Unlike React, Yew does not rerender when a sibling changes, so we have to force a rerender by changing a prop.
    #[prop_or_default]
    rerender_value: T,
}

#[function_component]
fn LogItems<T: Default + PartialEq = ()>(props: &LogItemsProps<T>) -> Html {
    let get_items = use_collection::<ItemData>();

    use_effect({
        let name = props.name.clone();
        let get_items = get_items.clone();

        move || {
            log::info!("{} {:?}", name, get_items.emit(()));
        }
    });

    Html::default()
}
