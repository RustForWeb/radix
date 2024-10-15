use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{Display, Formatter},
    rc::Rc,
};

use radix_number::clamp;
use radix_yew_collection::{
    use_collection, CollectionItemSlot, CollectionItemSlotChildProps, CollectionItemValue,
    CollectionProvider, CollectionSlot, CollectionSlotChildProps,
};
use radix_yew_direction::{use_direction, Direction};
use radix_yew_focus_guards::use_focus_guards;
use radix_yew_focus_scope::{FocusScope, FocusScopeChildProps};
use radix_yew_id::use_id;
use radix_yew_popper::{
    Align, Padding, Popper, PopperAnchor, PopperAnchorChildProps, PopperArrow,
    PopperArrowChildProps, PopperContent, PopperContentChildProps, SetPopperContentChildProps,
    Side, Sticky, UpdatePositionStrategy,
};
use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_yew_visually_hidden::{VisuallyHidden, VisuallyHiddenChildProps};
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    window,
};
use yew::{prelude::*, virtual_dom::VNode};

const OPEN_KEYS: [&str; 4] = [" ", "Enter", "ArrowUp", "ArrowDown"];
const SELECTION_KEYS: [&str; 2] = [" ", "Enter"];

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Position {
    #[default]
    ItemAligned,
    Popper,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::ItemAligned => "item-aligned",
                Position::Popper => "popper",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ItemData {
    value: String,
    disabled: bool,
    text_value: String,
}

#[derive(Clone, Debug, PartialEq)]
struct SelectContextValue {
    trigger_ref: NodeRef,
    value_node_ref: NodeRef,
    value_node_has_children: bool,
    on_value_node_has_children_change: Callback<bool>,
    content_id: String,
    value: Option<String>,
    on_value_change: Callback<String>,
    open: bool,
    required: Option<bool>,
    on_open_change: Callback<bool>,
    dir: Direction,
    trigger_pointer_down_pos_ref: Rc<RefCell<Option<(i32, i32)>>>,
    disabled: Option<bool>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct NativeOption {
    key: String,
    value: String,
    disabled: bool,
    text_content: Option<String>,
}

#[derive(Clone, PartialEq)]
struct SelectNativeOptionsContextValue {
    on_native_option_add: Callback<NativeOption>,
    on_native_option_remove: Callback<NativeOption>,
}

#[derive(PartialEq, Properties)]
pub struct SelectProps {
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub default_value: Option<String>,
    #[prop_or_default]
    pub on_value_change: Callback<String>,
    #[prop_or_default]
    pub open: Option<bool>,
    #[prop_or_default]
    pub default_open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub dir: Option<Direction>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub required: Option<bool>,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let trigger_ref = use_node_ref();
    let value_node_ref = use_node_ref();
    let value_node_has_children = use_state(|| false);
    let direction = use_direction(props.dir);

    let on_open_change = use_callback(
        props.on_open_change.clone(),
        |value: Option<bool>, on_open_change| {
            if let Some(value) = value {
                on_open_change.emit(value);
            }
        },
    );
    let (open, set_open) = use_controllable_state(UseControllableStateParams {
        prop: props.open,
        on_change: Some(on_open_change),
        default_prop: props.default_open,
    });
    let open = open.unwrap_or(false);
    let on_open_change = use_callback(set_open, |open: bool, set_open| {
        set_open.emit(Some(open));
    });

    let on_value_change = use_callback(
        props.on_value_change.clone(),
        |value: Option<String>, on_value_change| {
            if let Some(value) = value {
                on_value_change.emit(value);
            }
        },
    );
    let (value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: props.value.clone(),
        on_change: Some(on_value_change),
        default_prop: props.default_value.clone(),
    });
    let on_value_change = use_callback(set_value.clone(), |value: String, set_value| {
        set_value.emit(Some(value));
    });

    let trigger_pointer_down_pos_ref = use_mut_ref(|| None);
    let native_options_set = use_state_eq(HashSet::<NativeOption>::new);

    let is_form_control = use_state_eq(|| false);
    use_effect_with(trigger_ref.clone(), {
        let is_form_control = is_form_control.clone();

        move |trigger_ref| {
            is_form_control.set(
                trigger_ref
                    .cast::<web_sys::Element>()
                    .map(|button| button.closest("form").ok().flatten().is_some())
                    .unwrap_or(true),
            );
        }
    });

    // The native `select` only associates the correct default value if the corresponding
    // `option` is rendered as a child **at the same time** as itself.
    // Because it might take a few renders for our items to gather the information to build
    // the native `option`(s), we generate a key on the `select` to make sure Yew re-builds it
    // each time the options change.
    let native_select_key = native_options_set
        .iter()
        .map(|native_option| native_option.value.clone())
        .collect::<Vec<_>>()
        .join(";");

    let content_id = use_id(None);
    let context_value = use_memo(
        (
            props.disabled,
            props.required,
            value_node_has_children,
            direction,
            open,
            on_open_change,
            value.clone(),
            on_value_change,
            trigger_pointer_down_pos_ref,
        ),
        |(
            disabled,
            required,
            value_node_has_children,
            direction,
            open,
            on_open_change,
            value,
            on_value_change,
            trigger_pointer_down_pos_ref,
        )| {
            SelectContextValue {
                trigger_ref,
                value_node_ref,
                value_node_has_children: **value_node_has_children,
                on_value_node_has_children_change: Callback::from({
                    let value_node_has_children = value_node_has_children.clone();

                    move |has_children| value_node_has_children.set(has_children)
                }),
                content_id,
                value: value.clone(),
                on_value_change: on_value_change.clone(),
                open: *open,
                required: *required,
                on_open_change: on_open_change.clone(),
                dir: *direction,
                trigger_pointer_down_pos_ref: trigger_pointer_down_pos_ref.clone(),
                disabled: *disabled,
            }
        },
    );

    let native_options_context_value = use_memo((), {
        let native_options_set = native_options_set.clone();

        move |_| SelectNativeOptionsContextValue {
            on_native_option_add: Callback::from({
                let native_options_set = native_options_set.clone();

                move |option| {
                    let mut set = (*native_options_set).clone();
                    set.insert(option);
                    native_options_set.set(set);
                }
            }),
            on_native_option_remove: Callback::from({
                let native_options_set = native_options_set.clone();

                move |option| {
                    let mut set = (*native_options_set).clone();
                    set.remove(&option);
                    native_options_set.set(set);
                }
            }),
        }
    });

    html! {
        <Popper>
            <ContextProvider<SelectContextValue> context={(*context_value).clone()}>
                <CollectionProvider<ItemData>>
                    <ContextProvider<SelectNativeOptionsContextValue> context={(*native_options_context_value).clone()}>
                        {props.children.clone()}
                    </ContextProvider<SelectNativeOptionsContextValue>>
                </CollectionProvider<ItemData>>

                if *is_form_control {
                    <BubbleSelect
                        key={native_select_key}
                        name={props.name.clone()}
                        value={value.clone()}
                        required={props.required}
                        disabled={props.disabled}
                        form={props.form.clone()}
                        autocomplete={props.autocomplete.clone()}
                        tabindex="-1"
                        aria_hidden="true"
                        // Enable form autofill.
                        on_change={Callback::from(move |event: Event| {
                            set_value.emit(
                                event
                                    .target()
                                    .and_then(|target| target.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                    .map(|select_element| select_element.value())
                            )
                        })}
                    >
                        if value.is_none() {
                            <option value="" />
                        }
                        {
                            native_options_set.iter().map(|native_option| html! {
                                <option
                                    key={native_option.key.clone()}
                                    value={native_option.value.clone()}
                                    disabled={native_option.disabled}
                                >
                                    {native_option.text_content.clone().unwrap_or_default()}
                                </option>
                            }).collect::<Html>()
                        }
                    </BubbleSelect>
                }
            </ContextProvider<SelectContextValue>>
        </Popper>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectTriggerProps {
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_pointer_down: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectTriggerChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectTriggerChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub r#type: String,
    pub role: String,
    pub aria_controls: String,
    pub aria_expanded: String,
    pub aria_required: Option<String>,
    pub aria_autocomplete: String,
    pub dir: String,
    pub data_state: String,
    pub disabled: bool,
    pub data_disabled: Option<String>,
    pub data_placeholder: Option<String>,
    pub onclick: Callback<MouseEvent>,
    pub onpointerdown: Callback<PointerEvent>,
    pub onkeydown: Callback<KeyboardEvent>,
}

impl SelectTriggerChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <button
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                type={self.r#type}
                role={self.role}
                aria-controls={self.aria_controls}
                aria-expanded={self.aria_expanded}
                aria-required={self.aria_required}
                aria-autocomplete={self.aria_autocomplete}
                dir={self.dir}
                data-state={self.data_state}
                disabled={self.disabled}
                data-disabled={self.data_disabled}
                data-placeholder={self.data_placeholder}
                onclick={self.onclick}
                onpointerdown={self.onpointerdown}
                onkeydown={self.onkeydown}
            >
                {children}
            </button>
        }
    }
}

#[function_component]
pub fn SelectTrigger(props: &SelectTriggerProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let is_disabled = context.disabled.unwrap_or(props.disabled);
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), context.trigger_ref.clone()]);
    let get_items = use_collection::<ItemData>();
    let pointer_type_ref = use_mut_ref(|| "touch".to_string());

    let on_search_change = use_callback(
        (context.value.clone(), context.on_value_change, get_items),
        |search, (context_value, on_value_change, get_items)| {
            let enabled_items = get_items
                .emit(())
                .into_iter()
                .filter(|item| !item.data.disabled)
                .collect::<Vec<_>>();
            let current_item = enabled_items
                .iter()
                .find(|item| {
                    context_value
                        .as_ref()
                        .is_some_and(|value| item.data.value == *value)
                })
                .cloned();
            let next_item = find_next_item(enabled_items, search, current_item);
            if let Some(next_item) = next_item {
                on_value_change.emit(next_item.data.value);
            }
        },
    );
    let (search_ref, handle_typeahead_search, reset_typeahead) =
        use_typeahead_search(on_search_change);

    let handle_open = use_callback(
        (
            context.on_open_change,
            context.trigger_pointer_down_pos_ref,
            is_disabled,
        ),
        move |event_page_coords: Option<(i32, i32)>,
              (on_open_change, trigger_pointer_down_pos_ref, is_disabled)| {
            if !is_disabled {
                on_open_change.emit(true);
                // Reset typeahead when we open.
                reset_typeahead.emit(());
            }

            if let Some(event_page_coords) = event_page_coords {
                *trigger_pointer_down_pos_ref.borrow_mut() = Some(event_page_coords);
            }
        },
    );

    html! {
        <PopperAnchor
            node_ref={composed_refs}
            as_child={Callback::from({
                let id = props.id.clone();
                let class = props.class.clone();
                let style = props.style.clone();
                let on_click = props.on_click.clone();
                let on_pointer_down = props.on_pointer_down.clone();
                let on_key_down = props.on_key_down.clone();
                let as_child = props.as_child.clone();
                let children = props.children.clone();
                let content_id = context.content_id.clone();
                let value = context.value.clone();
                let pointer_type_ref = pointer_type_ref.clone();
                let handle_open = handle_open.clone();

                move |PopperAnchorChildProps { node_ref, .. }| {
                    let child_props = SelectTriggerChildProps {
                        node_ref,
                        id: id.clone(),
                        class: class.clone(),
                        style: style.clone(),
                        r#type: "button".into(),
                        role: "combobox".into(),
                        aria_controls: content_id.clone(),
                        aria_expanded: match context.open {
                            true => "true",
                            false => "false"
                        }.into(),
                        aria_required: context.required.map(|required| match required {
                            true => "true",
                            false => "false"
                        }.into()),
                        aria_autocomplete: "none".into(),
                        dir: context.dir.to_string(),
                        data_state: match context.open {
                            true => "open",
                            false => "closed"
                        }.into(),
                        disabled: is_disabled,
                        data_disabled: is_disabled.then_some("".into()),
                        data_placeholder: should_show_placeholder(value.clone()).then_some("".into()),
                        // Enable compatibility with native label or custom `Label` "click" for Safari:
                        onclick: compose_callbacks(Some(on_click.clone()), Some(Callback::from({
                            let pointer_type_ref = pointer_type_ref.clone();
                            let handle_open = handle_open.clone();

                            move |event: MouseEvent| {
                                // Whilst browsers generally have no issue focusing the trigger when clicking
                                // on a label, Safari seems to struggle with the fact that there's no `onclick`.
                                // We force `focus` in this case. Note: this doesn't create any other side-effect
                                // because we are preventing default in `onpointerdown` so effectively
                                // this only runs for a label "click".
                                event
                                    .current_target()
                                    .expect("Event should have current target.")
                                    .unchecked_into::<web_sys::HtmlElement>()
                                    .focus()
                                    .expect("Element should be focused.");

                                // Open on click when using a touch or pen device.
                                if *pointer_type_ref.borrow() != "mouse" {
                                    handle_open.emit(Some((event.page_x(), event.page_y())));
                                }
                        }})), None),
                        onpointerdown: compose_callbacks(Some(on_pointer_down.clone()), Some(Callback::from({
                            let pointer_type_ref = pointer_type_ref.clone();
                            let handle_open = handle_open.clone();

                            move |event: PointerEvent| {
                                *pointer_type_ref.borrow_mut() =event.pointer_type();

                                // Prevent implicit pointer capture.
                                // https://www.w3.org/TR/pointerevents3/#implicit-pointer-capture
                                let target = event.target().expect("Event should have target.").unchecked_into::<web_sys::HtmlElement>();
                                if target.has_pointer_capture(event.pointer_id()) {
                                    target.release_pointer_capture(event.pointer_id()).expect("Pointer capture should be released.");
                                }

                                // Only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
                                // but not when the control key is pressed (avoiding MacOS right click); also not for touch
                                // devices because that would open the menu on scroll. (pen devices behave as touch on iOS).
                                if event.button() == 0 && !event.ctrl_key() && event.pointer_type() == "mouse" {
                                    handle_open.emit(Some((event.page_x(), event.page_y())));

                                    // Prevent trigger from stealing focus from the active item after opening.
                                    event.prevent_default();
                                }
                            }
                        })), None),
                        onkeydown: compose_callbacks(Some(on_key_down.clone()), Some(Callback::from({
                            let handle_open = handle_open.clone();
                            let search_ref = search_ref.clone();
                            let handle_typeahead_search = handle_typeahead_search.clone();

                            move |event: KeyboardEvent| {
                                let is_typing_ahead = !search_ref.borrow().is_empty();
                                let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();

                                if !is_modifier_key && event.key().len() == 1 {
                                    handle_typeahead_search.emit(event.key());
                                }
                                if is_typing_ahead && event.key() == " " {
                                    return;
                                }
                                if OPEN_KEYS.contains(&event.key().as_str()) {
                                    handle_open.emit(None);
                                    event.prevent_default();
                                }
                            }
                        })), None)
                    };

                    if let Some(as_child) = as_child.as_ref() {
                        as_child.emit(child_props)
                    } else {
                        child_props.render(children.clone())
                    }
                }
            })}
        />
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectValueProps {
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectValueChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectValueChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: String,
}

impl SelectValueChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <span
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
            >
                {children}
            </span>
        }
    }
}

#[function_component]
pub fn SelectValue(props: &SelectValueProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), context.value_node_ref]);

    use_effect_with(props.children.clone(), {
        let on_value_node_has_children_change = context.on_value_node_has_children_change.clone();

        move |children| {
            let has_children = !matches!(children, VNode::VList(list) if list.is_empty());

            on_value_node_has_children_change.emit(has_children);
        }
    });

    let child_props = SelectValueChildProps {
        node_ref: composed_refs,
        id: props.id.clone(),
        class: props.class.clone(),
        // We don't want events from the portalled `SelectValue` children to bubble through the item they came from.
        style: format!(
            "pointer-events: none;{}",
            props.style.clone().unwrap_or_default()
        ),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render({
            if should_show_placeholder(context.value) {
                props.placeholder.clone().into()
            } else {
                props.children.clone()
            }
        })
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectIconProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectIconChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectIconChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub aria_hidden: String,
}

impl SelectIconChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <span
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                aria-hidden={self.aria_hidden}
            >
                {children}
            </span>
        }
    }
}

#[function_component]
pub fn SelectIcon(props: &SelectIconProps) -> Html {
    let child_props = SelectIconChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        aria_hidden: "true".into(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render({
            match &props.children {
                VNode::VList(list) if list.is_empty() => html! {"▼"},
                children => children.clone(),
            }
        })
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectPortalProps {
    // TODO: container
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectPortal(props: &SelectPortalProps) -> Html {
    html! {
        // TODO: Portal
        {props.children.clone()}
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectContentProps {
    // TODO
    #[prop_or(Position::ItemAligned)]
    pub position: Position,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    // TODO: change to SelectContentChildProps?
    pub as_child: Option<Callback<SelectContentImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectContent(props: &SelectContentProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let fragment = use_state_eq(|| None);

    use_effect_with((), {
        let fragment = fragment.clone();

        move |_| {
            fragment.set(Some(
                window()
                    .expect("Window should exist.")
                    .document()
                    .expect("Document should exist.")
                    .create_element("div")
                    .expect("Element should be created."),
            ));
        }
    });

    if context.open {
        html! {
            <SelectContentImpl
                position={props.position}
                id={props.id.clone()}
                class={props.class.clone()}
                style={props.style.clone()}
                node_ref={props.node_ref.clone()}
                as_child={props.as_child.clone()}
            >
                {props.children.clone()}
            </SelectContentImpl>
        }
    } else if let Some(fragment) = fragment.as_ref() {
        create_portal(
            html! {
                <ContextProvider<SelectContentContextValue> context={SelectContentContextValue::default()}>
                    <CollectionSlot<ItemData>
                        as_child={Callback::from({
                            let children = props.children.clone();

                            move |CollectionSlotChildProps { node_ref }| html! {
                                <div ref={node_ref}>
                                    {children.clone()}
                                </div>
                            }
                        })}
                    />
                </ContextProvider<SelectContentContextValue>>
            },
            fragment.clone(),
        )
    } else {
        Html::default()
    }
}

const CONTENT_MARGIN: f64 = 10.0;

#[derive(Clone, Default, PartialEq)]
struct SelectContentContextValue {
    content_ref: NodeRef,
    viewport_ref: NodeRef,
    item_ref_callback: Callback<(web_sys::HtmlElement, String, bool)>,
    selected_item: Option<web_sys::HtmlElement>,
    on_item_leave: Callback<()>,
    item_text_ref_callback: Callback<(web_sys::HtmlElement, String, bool)>,
    focus_selected_item: Callback<()>,
    selected_item_text: Option<web_sys::HtmlElement>,
    position: Position,
    is_positioned: bool,
    search_ref: Rc<RefCell<String>>,
}

#[derive(PartialEq, Properties)]
struct SelectContentImplProps {
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop_or_default]
    pub on_close_auto_focus: Callback<Event>,
    #[prop_or(Position::ItemAligned)]
    pub position: Position,
    #[prop_or(Side::Bottom)]
    pub side: Side,
    #[prop_or(0.0)]
    pub side_offset: f64,
    #[prop_or(Align::Start)]
    pub align: Align,
    #[prop_or(0.0)]
    pub align_offset: f64,
    #[prop_or(0.0)]
    pub arrow_padding: f64,
    #[prop_or(true)]
    pub avoid_collisions: bool,
    #[prop_or_default]
    pub collision_boundary: Vec<web_sys::Element>,
    #[prop_or(Padding::All(CONTENT_MARGIN))]
    pub collision_padding: Padding,
    #[prop_or(Sticky::Partial)]
    pub sticky: Sticky,
    #[prop_or(false)]
    pub hide_when_detached: bool,
    #[prop_or(UpdatePositionStrategy::Optimized)]
    pub update_position_strategy: UpdatePositionStrategy,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectContentImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectContentImplChildProps {
    pub item_aligned: Option<SelectItemAlignedPositionChildProps>,
    pub popper: Option<SelectPopperPositionChildProps>,
}

impl SetSelectItemAlignedPositionChildProps for SelectContentImplChildProps {
    fn set_select_item_aligned_position_props(
        &mut self,
        props: SelectItemAlignedPositionChildProps,
    ) {
        self.item_aligned = Some(props);
    }
}

impl SetSelectPopperPositionChildProps for SelectContentImplChildProps {
    fn set_select_popper_position_props(&mut self, props: SelectPopperPositionChildProps) {
        self.popper = Some(props);
    }
}

impl SetPopperContentChildProps for SelectContentImplChildProps {
    fn set_popper_content_child_props(&mut self, props: PopperContentChildProps) {
        let popper = self.popper.as_mut().expect("Popper should have a value.");
        popper.data_side = props.data_side;
        popper.data_align = props.data_align;
    }
}

#[function_component]
fn SelectContentImpl(props: &SelectContentImplProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select contenxt required.");
    let content_ref = use_node_ref();
    let viewport_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);
    let selected_item = use_state_eq(|| None);
    let selected_item_text = use_state_eq(|| None);
    let get_items = use_collection::<ItemData>();
    let is_positioned = use_state_eq(|| false);
    let first_valid_item_found_ref = use_mut_ref(|| false);

    // TODO

    // Make sure the whole tree has focus guards as our `Select` may be the last element in the DOM (because of the `Portal`).
    use_focus_guards();

    let focus_first = use_callback(
        (get_items.clone(), viewport_ref.clone()),
        |candidates: Vec<Option<web_sys::HtmlElement>>, (get_items, viewport_ref)| {
            let items = get_items
                .emit(())
                .into_iter()
                .map(|item| item.r#ref.cast::<web_sys::HtmlElement>())
                .collect::<Vec<_>>();
            let first_item = items.first().cloned().flatten();
            let last_item = items.last().cloned().flatten();

            let document = window()
                .expect("Window should exist.")
                .document()
                .expect("Document should exist.");
            let previously_focused_element = document.active_element();
            for candidate in candidates.into_iter().flatten() {
                // If focus is already where we want to go, we don't want to keep going through the candidates.
                if previously_focused_element
                    .as_ref()
                    .is_some_and(|previously_focused_element| {
                        *previously_focused_element == candidate.clone().into()
                    })
                {
                    return;
                }

                let options = web_sys::ScrollIntoViewOptions::new();
                options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                candidate.scroll_into_view_with_scroll_into_view_options(&options);

                // Viewport might have padding so scroll to it's edges when focusing first/last items.
                if first_item
                    .as_ref()
                    .is_some_and(|first_item| *first_item == candidate)
                {
                    let viewport = viewport_ref
                        .cast::<web_sys::HtmlElement>()
                        .expect("Viewport should exist.");
                    viewport.set_scroll_top(0);
                }
                if last_item
                    .as_ref()
                    .is_some_and(|last_item| *last_item == candidate)
                {
                    let viewport = viewport_ref
                        .cast::<web_sys::HtmlElement>()
                        .expect("Viewport should exist.");
                    viewport.set_scroll_top(viewport.scroll_height());
                }

                candidate.focus().expect("Element should be focused.");

                if document.active_element() != previously_focused_element {
                    return;
                }
            }
        },
    );

    let focus_selected_item = use_callback(
        (
            focus_first.clone(),
            selected_item.clone(),
            content_ref.clone(),
        ),
        |_: (), (focus_first, selected_item, content_ref)| {
            focus_first.emit(vec![
                (**selected_item).clone(),
                content_ref.cast::<web_sys::HtmlElement>(),
            ]);
        },
    );

    // Since this is not dependent on layout, we want to ensure this runs at the same time as
    // other effects across components. Hence why we don't call `focus_selected_tem` inside `position`.
    use_effect_with(is_positioned.clone(), {
        let focus_selected_item = focus_selected_item.clone();

        move |is_positioned| {
            if **is_positioned {
                focus_selected_item.emit(());
            }
        }
    });

    // Prevent selecting items on `pointerup` in some cases after opening from `pointerdown`
    // and close on `pointerup` outside.
    use_effect_with(
        (content_ref.clone(), context.trigger_pointer_down_pos_ref),
        |(content_ref, _trigger_pointer_down_pos_ref)| {
            if let Some(_content) = content_ref.get() {
                // TODO
            }
        },
    );

    use_effect({
        let on_open_change = context.on_open_change.clone();

        move || {
            let close_blur: Closure<dyn Fn(FocusEvent)> = Closure::new({
                let on_open_change = on_open_change.clone();

                move |_: FocusEvent| {
                    on_open_change.emit(false);
                }
            });
            let close_resize: Closure<dyn Fn(Event)> = Closure::new({
                let on_open_change = on_open_change.clone();

                move |_: Event| {
                    on_open_change.emit(false);
                }
            });

            let window = window().expect("Window should exist");
            window
                .add_event_listener_with_callback("blur", close_blur.as_ref().unchecked_ref())
                .expect("Blur event listener should be added.");
            window
                .add_event_listener_with_callback("resize", close_resize.as_ref().unchecked_ref())
                .expect("Resize event listener should be added.");

            move || {
                window
                    .remove_event_listener_with_callback(
                        "blur",
                        close_blur.as_ref().unchecked_ref(),
                    )
                    .expect("Blur event listener should be removed.");
                window
                    .remove_event_listener_with_callback(
                        "resize",
                        close_resize.as_ref().unchecked_ref(),
                    )
                    .expect("Resize event listener should be removed.");
            }
        }
    });

    let on_search_change = use_callback(get_items.clone(), |search, get_items| {
        let enabled_items = get_items
            .emit(())
            .into_iter()
            .filter(|item| !item.data.disabled)
            .collect::<Vec<_>>();
        let current_item = enabled_items
            .iter()
            .find(|item| {
                item.r#ref.cast::<web_sys::Element>()
                    == window()
                        .expect("Window should exist.")
                        .document()
                        .expect("Document should exist.")
                        .active_element()
            })
            .cloned();
        let next_item = find_next_item(enabled_items, search, current_item);
        if let Some(next_item) = next_item {
            next_item
                .r#ref
                .cast::<web_sys::HtmlElement>()
                .expect("Element should exist.")
                .focus()
                .expect("Element should be focused.");
        }
    });
    let (search_ref, handle_typeahead_search, _) = use_typeahead_search(on_search_change);

    let item_ref_callback = use_callback(context.value.clone(), {
        let first_valid_item_found_ref = first_valid_item_found_ref.clone();
        let selected_item = selected_item.clone();

        move |(node, value, disabled): (web_sys::HtmlElement, String, bool), context_value| {
            let is_first_valid_item = !*first_valid_item_found_ref.borrow() && !disabled;
            let is_selected_item = context_value
                .as_ref()
                .is_some_and(|context_value| *context_value == value);
            if is_selected_item || is_first_valid_item {
                selected_item.set(Some(node));

                if is_first_valid_item {
                    *first_valid_item_found_ref.borrow_mut() = true;
                }
            }
        }
    });
    let handle_item_leave = use_callback(content_ref.clone(), |_: (), content_ref| {
        if let Some(content) = content_ref.cast::<web_sys::HtmlElement>() {
            content.focus().expect("Element should be focused.");
        }
    });
    let item_text_ref_callback = use_callback(context.value, {
        let first_valid_item_found_ref = first_valid_item_found_ref.clone();
        let selected_item_text = selected_item_text.clone();

        move |(node, value, disabled): (web_sys::HtmlElement, String, bool), context_value| {
            let is_first_valid_item = !*first_valid_item_found_ref.borrow() && !disabled;
            let is_selected_item = context_value
                .as_ref()
                .is_some_and(|context_value| *context_value == value);
            if is_selected_item || is_first_valid_item {
                selected_item_text.set(Some(node));
            }
        }
    });

    let content_context_value = use_memo(
        (
            props.position,
            selected_item,
            selected_item_text,
            is_positioned.clone(),
        ),
        |(position, selected_item, selected_item_text, is_positioned)| SelectContentContextValue {
            content_ref,
            viewport_ref,
            item_ref_callback,
            selected_item: (**selected_item).clone(),
            on_item_leave: handle_item_leave,
            item_text_ref_callback,
            focus_selected_item,
            selected_item_text: (**selected_item_text).clone(),
            position: *position,
            is_positioned: **is_positioned,
            search_ref,
        },
    );

    html! {
        <ContextProvider<SelectContentContextValue> context={(*content_context_value).clone()}>
            // TODO: RemoveScrol, DismissableLayer

            <FocusScope
                node_ref={composed_refs}
                // We make sure we're not trapping once it's been closed
                // (closed !== unmounted when animating out).
                trapped={context.open}
                on_mount_auto_focus={Callback::from(|event: Event| {
                    // We prevent open autofocus because we manually focus the selected item.
                    event.prevent_default();
                })}
                on_unmount_auto_focus={compose_callbacks(Some(props.on_close_auto_focus.clone()), Some(Callback::from({
                    let trigger_ref = context.trigger_ref.clone();

                    move |event: Event| {
                        if let Some(trigger) = trigger_ref.cast::<web_sys::HtmlElement>() {
                            let options = web_sys::FocusOptions::new();
                            options.set_prevent_scroll(true);
                            trigger.focus_with_options(&options).expect("Element should be focused.");
                        }
                        event.prevent_default();
                    }
                })), None)}
                as_child={Callback::from({
                    let position = props.position;
                    let side = props.side;
                    let side_offset = props.side_offset;
                    let align = props.align;
                    let align_offset = props.align_offset;
                    let arrow_padding = props.arrow_padding;
                    let avoid_collisions = props.avoid_collisions;
                    let collision_boundary = props.collision_boundary.clone();
                    let collision_padding = props.collision_padding.clone();
                    let sticky = props.sticky;
                    let hide_when_detached = props.hide_when_detached;
                    let update_position_strategy = props.update_position_strategy;
                    let on_key_down = props.on_key_down.clone();
                    let id = props.id.clone().unwrap_or(context.content_id);
                    let class = props.class.clone();
                    let style = props.style.clone();
                    let as_child = props.as_child.clone();
                    let children = props.children.clone();
                    let is_positioned = is_positioned.clone();
                    let dir = context.dir.to_string();

                    move |FocusScopeChildProps {node_ref, onkeydown, ..}| {
                        // TODO: as_child_props?

                        let role = "listbox".to_string();
                        let data_state = if context.open {"open"} else {"closed"}.to_string();
                        let on_context_menu = Callback::from(|event: MouseEvent| event.prevent_default());

                        let on_placed = Callback::from({
                            let is_positioned = is_positioned.clone();

                            move |_| is_positioned.set(true)
                        });

                        // Flex layout so we can place the scroll buttons properly.
                        //
                        // Reset the outline by default as the content MAY get focused.
                        let style = format!("display: flex; flex-direction: column; outline: none;{}", style.clone().unwrap_or_default());

                        let on_key_down = compose_callbacks(Some(on_key_down.clone()), Some(Callback::from({
                            let get_items = get_items.clone();
                            let handle_typeahead_search = handle_typeahead_search.clone();
                            let focus_first = focus_first.clone();

                            move |event: KeyboardEvent| {
                                let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();

                                // Select should not be navigated using tab key so we prevent it.
                                if event.key() == "Tab" {
                                    event.prevent_default();
                                }

                                if !is_modifier_key && event.key().len() == 1 {
                                    handle_typeahead_search.emit(event.key());
                                }

                                if ["ArrowUp", "ArrowDown", "Home", "End"].contains(&event.key().as_str()) {
                                    let items = get_items.emit(()).into_iter().filter(|item| !item.data.disabled);
                                    let mut candidate_nodes = items
                                        .map(|item| item.r#ref.cast::<web_sys::HtmlElement>())
                                        .collect::<Vec<_>>();

                                    if ["ArrowUp", "End"].contains(&event.key().as_str()) {
                                        candidate_nodes.reverse();
                                    }
                                    if ["ArrowUp", "ArrowDown"].contains(&event.key().as_str()) {
                                        let current_element = event
                                            .target()
                                            .expect("Event should have target.")
                                            .dyn_into::<web_sys::HtmlElement>()
                                            .expect("Event target should be an HtmlElement.");
                                        let current_index = candidate_nodes.iter().position(|node| node.as_ref().is_some_and(|node| *node == current_element));
                                        candidate_nodes = candidate_nodes[current_index.map(|current_index| current_index + 1).unwrap_or(0)..].to_vec();
                                    }

                                    // TODO: set timeout?
                                    focus_first.emit(candidate_nodes);

                                    event.prevent_default();
                                }

                                onkeydown.emit(event);
                            }
                        })), None);

                        html! {
                            if position == Position::Popper {
                                <SelectPopperPosition<SelectContentImplChildProps>
                                    side={side}
                                    side_offset={side_offset}
                                    align={align}
                                    align_offset={align_offset}
                                    arrow_padding={arrow_padding}
                                    avoid_collisions={avoid_collisions}
                                    collision_boundary={collision_boundary.clone()}
                                    collision_padding={collision_padding.clone()}
                                    sticky={sticky}
                                    hide_when_detached={hide_when_detached}
                                    update_position_strategy={update_position_strategy}
                                    role={role}
                                    data_state={data_state}
                                    dir={dir.clone()}
                                    on_context_menu={on_context_menu}
                                    on_placed={on_placed}
                                    on_key_down={on_key_down}
                                    node_ref={node_ref}
                                    id={id.clone()}
                                    class={class.clone()}
                                    style={style}
                                    as_child={as_child.clone()}
                                >
                                    {children.clone()}
                                </SelectPopperPosition<SelectContentImplChildProps>>
                            } else {
                                <SelectItemAlignedPosition<SelectContentImplChildProps>
                                    role={role}
                                    data_state={data_state}
                                    dir={dir.clone()}
                                    on_context_menu={on_context_menu}
                                    on_placed={on_placed}
                                    on_key_down={on_key_down}
                                    node_ref={node_ref}
                                    id={id.clone()}
                                    class={class.clone()}
                                    style={style}
                                    as_child={as_child.clone()}
                                >
                                    {children.clone()}
                                </SelectItemAlignedPosition<SelectContentImplChildProps>>
                            }
                        }
                    }
                })}
            />
        </ContextProvider<SelectContentContextValue>>
    }
}

#[derive(PartialEq, Properties)]
struct SelectItemAlignedPositionProps<
    ChildProps: Clone + Default + PartialEq + SetSelectItemAlignedPositionChildProps,
> {
    #[prop_or_default]
    pub role: String,
    #[prop_or_default]
    pub data_state: String,
    #[prop_or_default]
    pub dir: Option<String>,
    #[prop_or_default]
    pub on_context_menu: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_placed: Callback<()>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<ChildProps, Html>>,
    #[prop_or_default]
    pub as_child_props: Option<ChildProps>,
    #[prop_or_default]
    pub children: Html,
}

pub trait SetSelectItemAlignedPositionChildProps {
    fn set_select_item_aligned_position_props(
        &mut self,
        props: SelectItemAlignedPositionChildProps,
    );
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectItemAlignedPositionChildProps {
    pub node_ref: NodeRef,
    pub id: String,
    pub class: Option<String>,
    pub style: String,
    pub role: String,
    pub data_state: String,
    pub dir: Option<String>,
    pub oncontextmenu: Callback<MouseEvent>,
    pub onkeydown: Callback<KeyboardEvent>,
}

impl SelectItemAlignedPositionChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                role={self.role}
                data-state={self.data_state}
                dir={self.dir}
                oncontextmenu={self.oncontextmenu}
                onkeydown={self.onkeydown}
            >
                {children}
            </div>
        }
    }
}

impl SetSelectItemAlignedPositionChildProps for SelectItemAlignedPositionChildProps {
    fn set_select_item_aligned_position_props(&mut self, _: SelectItemAlignedPositionChildProps) {}
}

#[function_component]
fn SelectItemAlignedPosition<ChildProps = SelectItemAlignedPositionChildProps>(
    props: &SelectItemAlignedPositionProps<ChildProps>,
) -> Html
where
    ChildProps: Clone + Default + PartialEq + SetSelectItemAlignedPositionChildProps,
{
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let content_wrapper_ref = use_node_ref();
    let content_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);
    let get_items = use_collection::<ItemData>();
    // TODO

    let position = use_callback(
        (
            get_items,
            context.trigger_ref,
            context.value_node_ref,
            content_wrapper_ref.clone(),
            content_ref,
            content_context.viewport_ref,
            content_context.selected_item,
            content_context.selected_item_text,
            context.dir,
            props.on_placed.clone(),
        ),
        |_: (),
         (
            get_items,
            trigger_ref,
            value_node_ref,
            content_wrapper_ref,
            content_ref,
            viewport_ref,
            selected_item,
            selected_item_text,
            dir,
            on_placed,
        )| {
            if let Some(trigger) = trigger_ref.cast::<web_sys::Element>() {
                if let Some(value_node) = value_node_ref.cast::<web_sys::Element>() {
                    if let Some(content_wrapper) =
                        content_wrapper_ref.cast::<web_sys::HtmlElement>()
                    {
                        if let Some(content) = content_ref.cast::<web_sys::Element>() {
                            if let Some(viewport) = viewport_ref.cast::<web_sys::HtmlElement>() {
                                if let Some(selected_item) = selected_item {
                                    if let Some(selected_item_text) = selected_item_text {
                                        let window = window().expect("Window should exist.");
                                        let window_inner_width = window
                                            .inner_width()
                                            .expect("Window should have inner width.")
                                            .as_f64()
                                            .expect("Inner width should be a number.");
                                        let window_inner_height = window
                                            .inner_height()
                                            .expect("Window should have inner height.")
                                            .as_f64()
                                            .expect("Inner height should be a number.");

                                        let trigger_rect = trigger.get_bounding_client_rect();

                                        // Horizontal positioning
                                        let content_rect = content.get_bounding_client_rect();
                                        let value_node_rect = value_node.get_bounding_client_rect();
                                        let item_text_rect =
                                            selected_item_text.get_bounding_client_rect();

                                        if *dir != Direction::Rtl {
                                            let item_text_offset =
                                                item_text_rect.left() - content_rect.left();
                                            let left = value_node_rect.left() - item_text_offset;
                                            let left_delta = trigger_rect.left() - left;
                                            let min_content_width =
                                                trigger_rect.width() + left_delta;
                                            let content_width =
                                                min_content_width.max(content_rect.width());
                                            let right_edge = window_inner_width - CONTENT_MARGIN;
                                            let clamped_left = clamp(
                                                left,
                                                [
                                                    CONTENT_MARGIN,
                                                    // Prevents the content from going off the starting edge of the
                                                    // viewport. It may still go off the ending edge, but this can be
                                                    // controlled by the user since they may want to manage overflow in a
                                                    // specific way.
                                                    // https://github.com/radix-ui/primitives/issues/2049
                                                    CONTENT_MARGIN.max(right_edge - content_width),
                                                ],
                                            );

                                            content_wrapper
                                                .style()
                                                .set_property(
                                                    "min-width",
                                                    &format!("{min_content_width}px"),
                                                )
                                                .expect("Min width should be set.");
                                            content_wrapper
                                                .style()
                                                .set_property("left", &format!("{clamped_left}px"))
                                                .expect("Left should be set.");
                                        } else {
                                            let item_text_offset =
                                                content_rect.right() - item_text_rect.right();
                                            let right = window_inner_width
                                                - value_node_rect.right()
                                                - item_text_offset;
                                            let right_delta =
                                                window_inner_width - trigger_rect.right() - right;
                                            let min_content_width =
                                                trigger_rect.width() + right_delta;
                                            let content_width =
                                                min_content_width.max(content_rect.width());
                                            let left_edge = window_inner_width - CONTENT_MARGIN;
                                            let clamped_right = clamp(
                                                right,
                                                [
                                                    CONTENT_MARGIN,
                                                    CONTENT_MARGIN.max(left_edge - content_width),
                                                ],
                                            );

                                            content_wrapper
                                                .style()
                                                .set_property(
                                                    "min-width",
                                                    &format!("{min_content_width}px"),
                                                )
                                                .expect("Min width should be set.");
                                            content_wrapper
                                                .style()
                                                .set_property("left", &format!("{clamped_right}px"))
                                                .expect("Left should be set.");
                                        }

                                        // Vertical positioning
                                        let items = get_items.emit(());
                                        let available_height =
                                            window_inner_height - CONTENT_MARGIN * 2.0;
                                        let items_height = viewport.scroll_height() as f64;

                                        let content_styles = window
                                            .get_computed_style(&content)
                                            .expect("Element is valid.")
                                            .expect("Element should have computed style.");
                                        let content_border_top_width = content_styles
                                            .get_property_value("border-top-width")
                                            .expect("Compyted style should have border top width.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Border top width should be a number.");
                                        let content_padding_top = content_styles
                                            .get_property_value("padding-top")
                                            .expect("Compyted style should have padding top.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding top should be a number.");
                                        let content_border_bottom_width = content_styles
                                            .get_property_value("border-bottom-width")
                                            .expect(
                                                "Compyted style should have border bottom width.",
                                            )
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Border bottom width should be a number.");
                                        let content_padding_bottom = content_styles
                                            .get_property_value("padding-bottom")
                                            .expect("Compyted style should have padding bottom.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding bottom should be a number.");
                                        let full_content_height = content_border_top_width
                                            + content_padding_top
                                            + items_height
                                            + content_padding_bottom
                                            + content_border_bottom_width;
                                        let min_content_height =
                                            (selected_item.offset_height() as f64 * 5.0)
                                                .min(full_content_height);

                                        let viewport_styles = window
                                            .get_computed_style(&viewport)
                                            .expect("Element is valid.")
                                            .expect("Element should have computed style.");
                                        let viewport_padding_top = viewport_styles
                                            .get_property_value("padding-top")
                                            .expect("Compyted style should have padding top.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding top should be a number.");
                                        let viewport_padding_bottom = viewport_styles
                                            .get_property_value("padding-bottom")
                                            .expect("Compyted style should have padding bottom.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding bottom should be a number.");

                                        let top_edge_to_trigger_middle = trigger_rect.top()
                                            + trigger_rect.height() / 2.0
                                            - CONTENT_MARGIN;
                                        let trigger_middle_to_bottom_edge =
                                            available_height - top_edge_to_trigger_middle;

                                        let selected_item_half_height =
                                            selected_item.offset_height() as f64 / 2.0;
                                        let item_offset_middle = selected_item.offset_top() as f64
                                            + selected_item_half_height;
                                        let content_top_to_item_middle = content_border_top_width
                                            - content_padding_top
                                            + item_offset_middle;
                                        let item_middle_to_content_bottom =
                                            full_content_height - content_top_to_item_middle;

                                        let will_align_without_top_overflow =
                                            content_top_to_item_middle
                                                <= top_edge_to_trigger_middle;

                                        if will_align_without_top_overflow {
                                            let is_last_item = !items.is_empty()
                                                && items
                                                    .last()
                                                    .expect("Last item should exist.")
                                                    .r#ref
                                                    .cast::<web_sys::HtmlElement>()
                                                    .is_some_and(|element| {
                                                        &element == selected_item
                                                    });

                                            content_wrapper
                                                .style()
                                                .set_property("bottom", "0px")
                                                .expect("Bottom should be set.");

                                            let viewport_offset_bottom = content.client_height()
                                                as f64
                                                - viewport.offset_top() as f64
                                                - viewport.offset_height() as f64;
                                            let clamped_trigger_middle_to_bottom_edge =
                                                trigger_middle_to_bottom_edge.max(
                                                    selected_item_half_height
                                                        // Viewport might have padding bottom,
                                                        // include it to avoid a scrollable viewport.
                                                        + match is_last_item {
                                                            true => viewport_padding_bottom,
                                                            false => 0.0,
                                                        }
                                                        + viewport_offset_bottom
                                                        + content_border_bottom_width,
                                                );
                                            let height = content_top_to_item_middle
                                                + clamped_trigger_middle_to_bottom_edge;

                                            content_wrapper
                                                .style()
                                                .set_property("height", &format!("{height}px"))
                                                .expect("Height should be set.");
                                        } else {
                                            let is_first_item = !items.is_empty()
                                                && items
                                                    .first()
                                                    .expect("First item should exist.")
                                                    .r#ref
                                                    .cast::<web_sys::HtmlElement>()
                                                    .is_some_and(|element| {
                                                        &element == selected_item
                                                    });

                                            content_wrapper
                                                .style()
                                                .set_property("top", "0px")
                                                .expect("Top should be set.");

                                            let clamped_top_edge_to_trigger_middle =
                                                top_edge_to_trigger_middle.max(
                                                    content_border_top_width
                                                        + viewport.offset_top() as f64
                                                        // Viewport might have padding top,
                                                        // include it to avoid a scrollable viewport.
                                                        + match is_first_item {
                                                            true => viewport_padding_top,
                                                            false => 0.0,
                                                        }
                                                        + selected_item_half_height,
                                                );
                                            let height = clamped_top_edge_to_trigger_middle
                                                + item_middle_to_content_bottom;

                                            content_wrapper
                                                .style()
                                                .set_property("height", &format!("{height}px"))
                                                .expect("Height should be set.");
                                            viewport.set_scroll_top(
                                                (content_top_to_item_middle
                                                    - top_edge_to_trigger_middle
                                                    + viewport.offset_top() as f64)
                                                    as i32,
                                            );
                                        }

                                        content_wrapper
                                            .style()
                                            .set_property(
                                                "margin",
                                                &format!("{CONTENT_MARGIN}px 0px"),
                                            )
                                            .expect("Margin should be set.");
                                        content_wrapper
                                            .style()
                                            .set_property(
                                                "min-height",
                                                &format!("{min_content_height}px"),
                                            )
                                            .expect("Min height should be set.");
                                        content_wrapper
                                            .style()
                                            .set_property(
                                                "max-height",
                                                &format!("{available_height}px"),
                                            )
                                            .expect("Min height should be set.");

                                        on_placed.emit(());

                                        // TODO: request animation frame
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    );

    use_effect(move || position.emit(()));

    // Copy z-index from content to wrapper.
    let content_z_index: UseStateHandle<Option<String>> = use_state_eq(|| None);
    // TODO

    let viewport_context_value = use_memo((), |_| SelectViewportContextValue {
        content_wrapper_ref: content_wrapper_ref.clone(),
    });

    let child_props = SelectItemAlignedPositionChildProps {
        node_ref: composed_refs,
        id: props.id.clone(),
        class: props.class.clone(),
        // When we get the height of the content, it includes borders. If we were to set
        // the height without having `box-sizing: border-box` it would be too big.
        //
        // We need to ensure the content doesn't get taller than the wrapper.
        style: format!(
            "box-sizing: border-box; max-height: 100%;{}",
            props.style.clone().unwrap_or_default()
        ),
        role: props.role.clone(),
        data_state: props.data_state.clone(),
        dir: props.dir.clone(),
        oncontextmenu: props.on_context_menu.clone(),
        onkeydown: props.on_key_down.clone(),
    };

    html! {
        <ContextProvider<SelectViewportContextValue> context={(*viewport_context_value).clone()}>
            <div
                ref={content_wrapper_ref}
                style={format!("display: flex; flex-direction: column; position: fixed;{}", content_z_index.as_ref().map(|content_z_index| format!("z-index: {content_z_index};")).unwrap_or_default())}
            >
                if let Some(as_child) = props.as_child.as_ref() {
                    {{
                        let mut as_child_props = props.as_child_props.clone().unwrap_or_default();
                        as_child_props.set_select_item_aligned_position_props(child_props);

                        as_child.emit(as_child_props)
                    }}
                } else {
                    {child_props.render(props.children.clone())}
                }
            </div>
        </ContextProvider<SelectViewportContextValue>>
    }
}

#[derive(PartialEq, Properties)]
struct SelectPopperPositionProps<
    ChildProps: Clone + Default + PartialEq + SetPopperContentChildProps + SetSelectPopperPositionChildProps,
> {
    #[prop_or(Side::Bottom)]
    pub side: Side,
    #[prop_or(0.0)]
    pub side_offset: f64,
    #[prop_or(Align::Start)]
    pub align: Align,
    #[prop_or(0.0)]
    pub align_offset: f64,
    #[prop_or(0.0)]
    pub arrow_padding: f64,
    #[prop_or(true)]
    pub avoid_collisions: bool,
    #[prop_or_default]
    pub collision_boundary: Vec<web_sys::Element>,
    #[prop_or(Padding::All(CONTENT_MARGIN))]
    pub collision_padding: Padding,
    #[prop_or(Sticky::Partial)]
    pub sticky: Sticky,
    #[prop_or(false)]
    pub hide_when_detached: bool,
    #[prop_or(UpdatePositionStrategy::Optimized)]
    pub update_position_strategy: UpdatePositionStrategy,
    #[prop_or_default]
    pub on_placed: Callback<()>,
    #[prop_or_default]
    pub dir: Option<String>,
    #[prop_or_default]
    pub role: String,
    #[prop_or_default]
    pub data_state: String,
    #[prop_or_default]
    pub on_context_menu: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<ChildProps, Html>>,
    #[prop_or_default]
    pub as_child_props: Option<ChildProps>,
    #[prop_or_default]
    pub children: Html,
}

pub trait SetSelectPopperPositionChildProps {
    fn set_select_popper_position_props(&mut self, props: SelectPopperPositionChildProps);
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectPopperPositionChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: String,
    pub role: String,
    pub data_side: String,
    pub data_align: String,
    pub data_state: String,
    pub dir: Option<String>,
    pub oncontextmenu: Callback<MouseEvent>,
    pub onkeydown: Callback<KeyboardEvent>,
}

impl SetSelectPopperPositionChildProps for SelectPopperPositionChildProps {
    fn set_select_popper_position_props(&mut self, _: SelectPopperPositionChildProps) {}
}

impl SetPopperContentChildProps for SelectPopperPositionChildProps {
    fn set_popper_content_child_props(&mut self, props: PopperContentChildProps) {
        self.node_ref = props.node_ref;
        self.id = props.id;
        self.class = props.class;
        self.style = props.style;
        self.role = props.role.expect("Prop `role` should always be set.");
        self.data_side = props.data_side;
        self.data_align = props.data_align;
        self.data_state = props
            .data_state
            .expect("Prop `data-state` should always be set.");
        self.oncontextmenu = props.oncontextmenu;
        self.onkeydown = props.onkeydown;
    }
}

#[function_component]
fn SelectPopperPosition<ChildProps = SelectPopperPositionChildProps>(
    props: &SelectPopperPositionProps<ChildProps>,
) -> Html
where
    ChildProps: Clone
        + Default
        + PartialEq
        + SetPopperContentChildProps
        + SetSelectPopperPositionChildProps
        + 'static,
{
    let child_props = SelectPopperPositionChildProps {
        ..SelectPopperPositionChildProps::default()
    };

    let mut as_child_props = props.as_child_props.clone().unwrap_or_default();
    as_child_props.set_select_popper_position_props(child_props);

    html! {
        <PopperContent<ChildProps>
            side={props.side}
            side_offset={props.side_offset}
            align={props.align}
            align_offset={props.align_offset}
            arrow_padding={props.arrow_padding}
            avoid_collisions={props.avoid_collisions}
            collision_boundary={props.collision_boundary.clone()}
            collision_padding={props.collision_padding.clone()}
            sticky={props.sticky}
            hide_when_detached={props.hide_when_detached}
            update_position_strategy={props.update_position_strategy}
            on_placed={props.on_placed.clone()}
            dir={props.dir.clone()}
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={props.class.clone()}
            style={format!(
                // Ensure border-box for Floating UI calculations.
                // Re-namespace exposed content custom properties.
                "\
                box-sizing: border-box;\
                --radix-select-content-transform-origin: var(--radix-popper-transform-origin);\
                --radix-select-content-available-width: var(--radix-popper-available-width);\
                --radix-select-content-available-height: var(--radix-popper-available-height);\
                --radix-select-trigger-width: var(--radix-popper-anchor-width);\
                --radix-select-trigger-height: var(--radix-popper-anchor-height);\
                {}",
                props.style.clone().unwrap_or_default()
            )}
            role={props.role.clone()}
            data_state={props.data_state.clone()}
            on_context_menu={props.on_context_menu.clone()}
            on_key_down={props.on_key_down.clone()}
            as_child={props.as_child.clone()}
            as_child_props={as_child_props}
        >
            {props.children.clone()}
        </PopperContent<ChildProps>>
    }
}

#[derive(Clone, PartialEq)]
struct SelectViewportContextValue {
    content_wrapper_ref: NodeRef,
    // TODO
    // should_expand_on_scroll: bool,
    // on_scroll_button_change: Callback<>
}

#[derive(PartialEq, Properties)]
pub struct SelectViewportProps {
    #[prop_or_default]
    pub nonce: Option<String>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectViewportChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectViewportChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: String,
    pub data_radix_select_viewport: String,
    pub role: String,
}

impl SelectViewportChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            html! {
                <div
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    data-radix-select-viewport={self.data_radix_select_viewport}
                    role={self.role}
                    style={self.style}
                >
                    {children}
                </div>
            }
        }
    }
}

#[function_component]
pub fn SelectViewport(props: &SelectViewportProps) -> Html {
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_context.viewport_ref]);

    html! {
        <>
            // Hide scrollbars cross-browser and enable momentum scroll for touch devices.
            <style nonce={props.nonce.clone()}>
                {"[data-radix-select-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}\
                [data-radix-select-viewport]::-webkit-scrollbar{display:none;}"}
            </style>

            <CollectionSlot<ItemData>
                node_ref={composed_refs}
                as_child={Callback::from({
                    let id = props.id.clone();
                    let class = props.class.clone();
                    let style = props.style.clone();
                    let as_child = props.as_child.clone();
                    let children = props.children.clone();

                    move |CollectionSlotChildProps { node_ref }| {
                        let child_props = SelectViewportChildProps {
                            node_ref,
                            id: id.clone(),
                            class: class.clone(),
                            data_radix_select_viewport: "".into(),
                            role: "presentation".into(),
                            // We use position: 'relative' here on the `viewport` so that when we call `selected_item.offset_top` in calculations,
                            // the offset is relative to the viewport (independent of the ScrollUpButton).
                            style: format!("position: relative; flex: 1; overflow: auto;{}", style.clone().unwrap_or_default())
                        };

                        if let Some(as_child) = as_child.as_ref() {
                            as_child.emit(child_props)
                        } else {
                            child_props.render(children.clone())
                        }
                    }
                })}
            />
        </>
    }
}

#[derive(Clone, PartialEq)]
struct SelectGroupContextValue {
    id: String,
}

#[derive(PartialEq, Properties)]
pub struct SelectGroupProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectGroupChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectGroupChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub role: String,
    pub aria_labelledby: String,
}

impl SelectGroupChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                role={self.role}
                aria-labelledby={self.aria_labelledby}
            >
                {children}
            </div>
        }
    }
}

#[function_component]
pub fn SelectGroup(props: &SelectGroupProps) -> Html {
    let group_id = use_id(None);

    let context_value = use_memo(group_id.clone(), |group_id| SelectGroupContextValue {
        id: group_id.clone(),
    });

    let child_props = SelectGroupChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        role: "group".into(),
        aria_labelledby: group_id,
    };

    html! {
        <ContextProvider<SelectGroupContextValue> context={(*context_value).clone()}>
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        </ContextProvider<SelectGroupContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectLabelProps {
    // TODO
    #[prop_or_default]
    pub node_ref: NodeRef,
    // TODO: can be removed because generated?
    // #[prop_or_default]
    // pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectLabelChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectLabelChildProps {
    pub node_ref: NodeRef,
    pub id: String,
    pub class: Option<String>,
    pub style: Option<String>,
}

impl SelectLabelChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
            >
                {children}
            </div>
        }
    }
}

#[function_component]
pub fn SelectLabel(props: &SelectLabelProps) -> Html {
    let group_context =
        use_context::<SelectGroupContextValue>().expect("Select group context required.");

    let child_props = SelectLabelChildProps {
        node_ref: props.node_ref.clone(),
        id: group_context.id,
        class: props.class.clone(),
        style: props.style.clone(),
        // TODO
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(Clone, PartialEq)]
struct SelectItemContextValue {
    value: String,
    disabled: bool,
    text_id: String,
    is_selected: bool,
    on_item_text_change: Callback<web_sys::HtmlElement>,
}

#[derive(PartialEq, Properties)]
pub struct SelectItemProps {
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub text_value: String,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_pointer_up: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_down: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_move: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_leave: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectItemChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectItemChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub data_radix_collection_item: String,
    pub role: String,
    pub aria_labelledby: String,
    pub data_highlighted: Option<String>,
    pub aria_selected: Option<String>,
    pub data_state: String,
    pub aria_disabled: Option<String>,
    pub data_disabled: Option<String>,
    pub tabindex: Option<String>,
    pub onfocus: Callback<FocusEvent>,
    pub onblur: Callback<FocusEvent>,
    pub onclick: Callback<MouseEvent>,
    pub onpointerup: Callback<PointerEvent>,
    pub onpointerdown: Callback<PointerEvent>,
    pub onpointermove: Callback<PointerEvent>,
    pub onpointerleave: Callback<PointerEvent>,
    pub onkeydown: Callback<KeyboardEvent>,
}

impl SelectItemChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                data-radix-collection-item={self.data_radix_collection_item}
                role={self.role}
                aria-labelledby={self.aria_labelledby}
                data-highlighted={self.data_highlighted}
                aria-selected={self.aria_selected}
                data-state={self.data_state}
                aria-disabled={self.aria_disabled}
                data-disabled={self.data_disabled}
                tabindex={self.tabindex}
                onfocus={self.onfocus}
                onblur={self.onblur}
                onclick={self.onclick}
                onpointerup={self.onpointerup}
                onpointerdown={self.onpointerdown}
                onpointermove={self.onpointermove}
                onpointerleave={self.onpointerleave}
                onkeydown={self.onkeydown}
            >
                {children}
            </div>
        }
    }
}

#[function_component]
pub fn SelectItem(props: &SelectItemProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let text_value = use_state_eq(|| props.text_value.clone());
    let is_focused = use_state_eq(|| false);
    let item_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), item_ref.clone()]);
    let is_selected = context.value.is_some_and(|value| value == props.value);
    let text_id = use_id(None);
    let pointer_type_ref = use_mut_ref(|| "touch".to_string());

    let item_ref_callback = use_callback(
        (
            content_context.item_ref_callback,
            props.value.clone(),
            props.disabled,
        ),
        |node: web_sys::HtmlElement, (item_ref_callback, value, disabled)| {
            item_ref_callback.emit((node, value.clone(), *disabled))
        },
    );
    use_effect_with(item_ref.clone(), move |item_ref| {
        if let Some(node) = item_ref.cast::<web_sys::HtmlElement>() {
            item_ref_callback.emit(node);
        }
    });

    let handle_select = use_callback(
        (
            props.disabled,
            props.value.clone(),
            context.on_value_change,
            context.on_open_change,
        ),
        move |_: (), (disabled, value, on_value_change, on_open_change)| {
            if !*disabled {
                on_value_change.emit(value.clone());
                on_open_change.emit(false);
            }
        },
    );

    let on_item_text_change = use_callback((), {
        let text_value = text_value.clone();

        move |node: web_sys::HtmlElement, _| {
            text_value.set(if text_value.is_empty() {
                node.text_content()
                    .map(|text_content| text_content.trim().to_string())
                    .unwrap_or_default()
            } else {
                (*text_value).clone()
            });
        }
    });

    let item_context_value = use_memo(
        (
            props.value.clone(),
            props.disabled,
            text_id.clone(),
            is_selected,
        ),
        |(value, disabled, text_id, is_selected)| SelectItemContextValue {
            value: (*value).clone(),
            disabled: *disabled,
            text_id: (*text_id).clone(),
            is_selected: *is_selected,
            on_item_text_change,
        },
    );

    let item_data = use_memo(
        (props.value.clone(), props.disabled, text_value),
        |(value, disabled, text_value)| ItemData {
            value: (*value).clone(),
            disabled: *disabled,
            text_value: (**text_value).clone(),
        },
    );

    let onfocus = compose_callbacks(
        Some(props.on_focus.clone()),
        Some(Callback::from({
            let is_focused = is_focused.clone();

            move |_: FocusEvent| is_focused.set(true)
        })),
        None,
    );
    let onblur = compose_callbacks(
        Some(props.on_blur.clone()),
        Some(Callback::from({
            let is_focused = is_focused.clone();

            move |_: FocusEvent| is_focused.set(false)
        })),
        None,
    );
    let onclick = compose_callbacks(
        Some(props.on_click.clone()),
        Some(Callback::from({
            let pointer_type_ref = pointer_type_ref.clone();
            let handle_select = handle_select.clone();

            move |_: MouseEvent| {
                // Open on click when using a touch or pen device.
                if *pointer_type_ref.borrow() != "mouse" {
                    handle_select.emit(());
                }
            }
        })),
        None,
    );
    let onpointerup = compose_callbacks(
        Some(props.on_pointer_up.clone()),
        Some(Callback::from({
            let pointer_type_ref = pointer_type_ref.clone();
            let handle_select = handle_select.clone();

            move |_: PointerEvent| {
                // Using a mouse you should be able to do pointer down, move through
                // the list, and release the pointer over the item to select it.
                if *pointer_type_ref.borrow() == "mouse" {
                    handle_select.emit(());
                }
            }
        })),
        None,
    );
    let onpointerdown = compose_callbacks(
        Some(props.on_pointer_down.clone()),
        Some(Callback::from({
            let pointer_type_ref = pointer_type_ref.clone();

            move |event: PointerEvent| {
                *pointer_type_ref.borrow_mut() = event.pointer_type();
            }
        })),
        None,
    );
    let onpointermove = compose_callbacks(
        Some(props.on_pointer_move.clone()),
        Some(Callback::from({
            let item_ref = item_ref.clone();
            let pointer_type_ref = pointer_type_ref.clone();
            let disabled = props.disabled;
            let on_item_leave = content_context.on_item_leave.clone();

            move |event: PointerEvent| {
                // Remember pointer type when sliding over to this item from another one.
                *pointer_type_ref.borrow_mut() = event.pointer_type();

                if disabled {
                    on_item_leave.emit(());
                } else if *pointer_type_ref.borrow() == "mouse" {
                    // Even though Safari doesn't support this option, it's acceptable
                    // as it only means it might scroll a few pixels when using the pointer.
                    let options = web_sys::FocusOptions::new();
                    options.set_prevent_scroll(true);

                    // Yew messes up `current_target`, see https://yew.rs/docs/concepts/html/events#event-delegation.
                    //
                    // event
                    //     .current_target()
                    //     .expect("Event should have target.")
                    //     .unchecked_into::<web_sys::HtmlElement>()
                    item_ref
                        .cast::<web_sys::HtmlElement>()
                        .expect("Item should exist.")
                        .focus_with_options(&options)
                        .expect("Element should be focused.");
                }
            }
        })),
        None,
    );
    let onpointerleave = compose_callbacks(
        Some(props.on_pointer_leave.clone()),
        Some(Callback::from({
            let item_ref = item_ref.clone();
            let on_item_leave = content_context.on_item_leave.clone();

            move |_event: PointerEvent| {
                // Yew messes up `current_target`, see https://yew.rs/docs/concepts/html/events#event-delegation.
                //
                // event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::Element>());
                if item_ref.cast::<web_sys::Element>()
                    != window()
                        .expect("Window should exist.")
                        .document()
                        .expect("Document should exist.")
                        .active_element()
                {
                    on_item_leave.emit(());
                }
            }
        })),
        None,
    );
    let onkeydown = compose_callbacks(
        Some(props.on_key_down.clone()),
        Some(Callback::from({
            move |event: KeyboardEvent| {
                let is_typing_ahead = !content_context.search_ref.borrow().is_empty();
                if is_typing_ahead && event.key() == " " {
                    return;
                }
                if SELECTION_KEYS.contains(&event.key().as_str()) {
                    handle_select.emit(());
                }
                // Prevent page scroll if using the space key to select an item.
                if event.key() == " " {
                    event.prevent_default();
                }
            }
        })),
        None,
    );

    html! {
        <ContextProvider<SelectItemContextValue> context={(*item_context_value).clone()}>
            <CollectionItemSlot<ItemData>
                node_ref={composed_refs}
                item_data={(*item_data).clone()}
                as_child={Callback::from({
                    let disabled = props.disabled;
                    let id = props.id.clone();
                    let class = props.class.clone();
                    let style = props.style.clone();
                    let as_child = props.as_child.clone();
                    let children = props.children.clone();
                    let is_focused = is_focused.clone();

                    move |CollectionItemSlotChildProps { node_ref, data_radix_collection_item }| {
                        let child_props = SelectItemChildProps {
                            node_ref,
                            id: id.clone(),
                            class: class.clone(),
                            style: style.clone(),
                            data_radix_collection_item,
                            role: "option".into(),
                            aria_labelledby: text_id.clone(),
                            data_highlighted: is_focused.then_some("".into()),
                            // `is_focused` caveat fixes stuttering in VoiceOver.
                            aria_selected: (is_selected && *is_focused).then_some("true".into()),
                            data_state: (if is_selected { "checked" } else { "unchecked "}).into(),
                            aria_disabled: disabled.then_some("true".into()),
                            data_disabled: disabled.then_some("".into()),
                            tabindex: (!disabled).then_some("-1".into()),
                            onfocus: onfocus.clone(),
                            onblur: onblur.clone(),
                            onclick: onclick.clone(),
                            onpointerup: onpointerup.clone(),
                            onpointerdown: onpointerdown.clone(),
                            onpointermove: onpointermove.clone(),
                            onpointerleave: onpointerleave.clone(),
                            onkeydown: onkeydown.clone(),
                        };

                        if let Some(as_child) = as_child.as_ref() {
                            as_child.emit(child_props)
                        } else {
                            child_props.render(children.clone())
                        }
                    }
                })}
            />
        </ContextProvider<SelectItemContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectItemTextProps {
    // TODO
    #[prop_or_default]
    pub node_ref: NodeRef,
    // TODO: can be removed because generated?
    // #[prop_or_default]
    // pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectItemTextChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectItemTextChildProps {
    pub node_ref: NodeRef,
    pub id: String,
    pub class: Option<String>,
    pub style: Option<String>,
}

impl SelectItemTextChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <span
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
            >
                {children}
            </span>
        }
    }
}

#[function_component]
pub fn SelectItemText(props: &SelectItemTextProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let item_context =
        use_context::<SelectItemContextValue>().expect("Select item context required.");
    let native_options_context = use_context::<SelectNativeOptionsContextValue>()
        .expect("Select native options context required.");
    let item_text_node_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), item_text_node_ref.clone()]);

    let item_text_ref_callback = use_callback(
        (content_context.item_text_ref_callback, item_context.clone()),
        |node: web_sys::HtmlElement, (item_text_ref_callback, item_context)| {
            item_context.on_item_text_change.emit(node.clone());
            item_text_ref_callback.emit((node, item_context.value.clone(), item_context.disabled));
        },
    );
    use_effect_with(item_text_node_ref.clone(), move |item_text_node_ref| {
        if let Some(node) = item_text_node_ref.cast::<web_sys::HtmlElement>() {
            item_text_ref_callback.emit(node);
        }
    });

    let text_content = use_memo(item_text_node_ref, |item_text_node_ref| {
        item_text_node_ref
            .get()
            .and_then(|item_text_node| item_text_node.text_content())
    });
    let native_option = use_memo(
        (item_context.disabled, item_context.value, text_content),
        |(disabled, value, text_content)| NativeOption {
            key: value.clone(),
            value: value.clone(),
            disabled: *disabled,
            text_content: (**text_content).clone(),
        },
    );

    use_effect_with(
        (
            native_option,
            native_options_context.on_native_option_add,
            native_options_context.on_native_option_remove,
        ),
        |(native_option, on_native_option_add, on_native_option_remove)| {
            let native_option = (**native_option).clone();
            on_native_option_add.emit(native_option.clone());

            {
                let on_native_option_remove = on_native_option_remove.clone();

                move || on_native_option_remove.emit(native_option)
            }
        },
    );

    let child_props = SelectItemTextChildProps {
        node_ref: composed_refs,
        id: item_context.text_id,
        class: props.class.clone(),
        style: props.style.clone(),
        // TODO
    };

    html! {
        <>
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }

            if item_context.is_selected && !context.value_node_has_children {
                if let Some(value_node) = context.value_node_ref.cast::<web_sys::Element>() {
                    {create_portal(props.children.clone(), value_node)}
                }
            }
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectItemIndicatorProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectItemIndicatorChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectItemIndicatorChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub aria_hidden: String,
}

impl SelectItemIndicatorChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <span
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                aria-hidden={self.aria_hidden}
            >
                {children}
            </span>
        }
    }
}

#[function_component]
pub fn SelectItemIndicator(props: &SelectItemIndicatorProps) -> Html {
    let item_context =
        use_context::<SelectItemContextValue>().expect("Select item context required.");

    let child_props = SelectItemIndicatorChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        aria_hidden: "true".into(),
    };

    html! {
        if item_context.is_selected {
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectScrollUpButtonProps {
    // TODO
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    // TODO: change to SelectScrollUpButtonChildProps
    pub as_child: Option<Callback<SelectScrollButtonImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectScrollUpButton(props: &SelectScrollUpButtonProps) -> Html {
    // TODO
    html! {
        <SelectScrollButtonImpl
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={props.class.clone()}
            style={props.style.clone()}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
        </SelectScrollButtonImpl>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectScrollDownButtonProps {
    // TODO
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    // TODO: change to SelectScrollDownButtonChildProps
    pub as_child: Option<Callback<SelectScrollButtonImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectScrollDownButton(props: &SelectScrollDownButtonProps) -> Html {
    // TODO
    html! {
        <SelectScrollButtonImpl
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={props.class.clone()}
            style={props.style.clone()}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
        </SelectScrollButtonImpl>
    }
}

#[derive(PartialEq, Properties)]
struct SelectScrollButtonImplProps {
    // TODO
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectScrollButtonImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectScrollButtonImplChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub aria_hidden: String,
}

impl SelectScrollButtonImplChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                aria-hidden={self.aria_hidden}
            >
                {children}
            </div>
        }
    }
}

#[function_component]
fn SelectScrollButtonImpl(props: &SelectScrollButtonImplProps) -> Html {
    let _content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let _get_items = use_collection::<ItemData>();

    let child_props = SelectScrollButtonImplChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        aria_hidden: "true".into(),
        // TODO
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectSeparatorProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SelectSeparatorChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SelectSeparatorChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub aria_hidden: String,
}

impl SelectSeparatorChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                aria-hidden={self.aria_hidden}
            >
                {children}
            </div>
        }
    }
}

#[function_component]
pub fn SelectSeparator(props: &SelectSeparatorProps) -> Html {
    let child_props = SelectSeparatorChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        aria_hidden: "true".into(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectArrowProps {
    #[prop_or(10.0)]
    pub width: f64,
    #[prop_or(5.0)]
    pub height: f64,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<PopperArrowChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectArrow(props: &SelectArrowProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");

    html! {
        if context.open && content_context.position == Position::Popper {
            <PopperArrow
                width={props.width}
                height={props.height}
                node_ref={props.node_ref.clone()}
                id={props.id.clone()}
                class={props.class.clone()}
                style={props.style.clone()}
                as_child={props.as_child.clone()}
            >
                {props.children.clone()}
            </PopperArrow>
        }
    }
}
fn should_show_placeholder(value: Option<String>) -> bool {
    value.is_none_or(|value| value.is_empty())
}

#[derive(PartialEq, Properties)]
struct BubbleSelectProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub required: Option<bool>,
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub tabindex: Option<String>,
    #[prop_or_default]
    pub aria_hidden: Option<String>,
    #[prop_or_default]
    pub on_change: Callback<Event>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn BubbleSelect(props: &BubbleSelectProps) -> Html {
    html! {
        <VisuallyHidden
            id={props.id.clone()}
            class={props.class.clone()}
            style={props.style.clone()}
            as_child={Callback::from({
                let name = props.name.clone();
                let value = props.value.clone();
                let required = props.required;
                let disabled = props.disabled;
                let form = props.form.clone();
                let autocomplete = props.autocomplete.clone();
                let tabindex = props.tabindex.clone();
                let aria_hidden = props.aria_hidden.clone();
                let on_change = props.on_change.clone();
                let children = props.children.clone();

                move |VisuallyHiddenChildProps {node_ref, id, class, style}| html! {
                    <select
                        ref={node_ref}
                        id={id}
                        class={class}
                        style={style}
                        name={name.clone()}
                        value={value.clone()}
                        required={required.unwrap_or(false)}
                        disabled={disabled.unwrap_or(false)}
                        form={form.clone()}
                        autocomplete={autocomplete.clone()}
                        tabindex={tabindex.clone()}
                        aria-hidden={aria_hidden.clone()}
                        onchange={on_change.clone()}
                    >
                        {children.clone()}
                    </select>
                }
            })}
        />
    }
}

#[hook]
fn use_typeahead_search(
    on_search_change: Callback<String>,
) -> (Rc<RefCell<String>>, Callback<String>, Callback<()>) {
    let search_ref = use_mut_ref(|| "".to_string());
    let timer_ref = use_mut_ref(|| 0);

    let clear_search: Closure<dyn Fn()> = Closure::new({
        let search_ref = search_ref.clone();

        move || {
            *search_ref.borrow_mut() = "".to_string();
        }
    });

    let handle_typeahead_search = use_callback(on_search_change, {
        let search_ref = search_ref.clone();
        let timer_ref = timer_ref.clone();

        move |key, on_search_change| {
            let search = format!("{}{}", search_ref.borrow(), key);
            on_search_change.emit(search.clone());

            *search_ref.borrow_mut() = search.clone();

            let window = window().expect("Window should exist.");
            window.clear_timeout_with_handle(*timer_ref.borrow());

            if !search.is_empty() {
                // Reset `search_ref` 1 second after it was last updated.
                *timer_ref.borrow_mut() = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        clear_search.as_ref().unchecked_ref(),
                        1000,
                    )
                    .expect("Timeout should be set.")
            }
        }
    });

    let reset_typeahead = use_callback((), {
        let search_ref = search_ref.clone();
        let timer_ref = timer_ref.clone();

        move |_, _| {
            *search_ref.borrow_mut() = "".to_string();

            window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(*timer_ref.borrow());
        }
    });

    use_effect(move || {
        move || {
            window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(*timer_ref.borrow());
        }
    });

    (search_ref, handle_typeahead_search, reset_typeahead)
}

/// This is the "meat" of the typeahead matching logic. It takes in a list of items,
/// the search and the current item, and returns the next item (or `None`).
///
/// We normalize the search because if a user has repeatedly pressed a character,
/// we want the exact same behavior as if we only had that one character
/// (ie. cycle through items starting with that character).
///
/// We also reorder the items by wrapping the array around the current item.
/// This is so we always look forward from the current item, and picking the first
/// item will always be the correct one.
///
/// Finally, if the normalized search is exactly one character, we exclude the
/// current item from the values because otherwise it would be the first to match always
/// and focus would never move. This is as opposed to the regular case, where we
/// don't want focus to move if the current item still matches.
fn find_next_item(
    items: Vec<CollectionItemValue<ItemData>>,
    search: String,
    current_item: Option<CollectionItemValue<ItemData>>,
) -> Option<CollectionItemValue<ItemData>> {
    let is_repeated = search.chars().count() > 1
        && search.chars().all(|char| {
            char == search
                .chars()
                .next()
                .expect("String is at least one character long.")
        });
    let normalized_search = if is_repeated {
        search.chars().take(1).collect()
    } else {
        search
    };
    let current_item_index = current_item
        .as_ref()
        .and_then(|current_item| items.iter().position(|item| item == current_item));
    let mut wrapped_items =
        wrap_array(&mut items.clone(), current_item_index.unwrap_or(0)).to_vec();
    let exclude_current_item = normalized_search.chars().count() == 1;
    if exclude_current_item {
        wrapped_items.retain(|item| {
            current_item
                .as_ref()
                .is_none_or(|current_item| item != current_item)
        });
    }
    let next_item = wrapped_items.into_iter().find(|item| {
        item.data
            .text_value
            .to_lowercase()
            .starts_with(&normalized_search.to_lowercase())
    });

    if next_item != current_item {
        next_item
    } else {
        None
    }
}

/// Wraps an array around itself at a given start index.
fn wrap_array<T: Clone>(array: &mut [T], start_index: usize) -> &[T] {
    array.rotate_right(start_index);
    array
}
