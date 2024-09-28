use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use radix_yew_compose_refs::use_composed_refs;
use radix_yew_direction::{use_direction, Direction};
use radix_yew_id::use_id;
use radix_yew_popper::{Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_yew_primitive::{compose_callbacks, Primitive};
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use web_sys::wasm_bindgen::JsCast;
use yew::{prelude::*, virtual_dom::VNode};
use yew_attrs::{attrs, Attrs};

const OPEN_KEYS: [&str; 4] = [" ", "Enter", "ArrowUp", "ArrowDown"];
const _SELECTION_KEYS: [&str; 2] = [" ", "Enter"];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Position {
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

#[derive(Clone, PartialEq)]
struct SelectContextValue {
    trigger_ref: NodeRef,
    value_node_ref: NodeRef,
    // TODO: value_node_has_children?
    content_id: String,
    value: Option<String>,
    on_value_change: Callback<String>,
    open: bool,
    required: Option<bool>,
    on_open_change: Callback<bool>,
    dir: Direction,
    trigger_pointer_down_pos: UseStateHandle<Option<(i32, i32)>>,
    disabled: Option<bool>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct NativeOption {
    key: String,
    value: String,
    disabled: bool,
    text_content: String,
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
    pub children: Children,
}

#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let trigger_ref = use_node_ref();
    let value_node_ref = use_node_ref();
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
    let on_value_change = use_callback(set_value, |value: String, set_value| {
        set_value.emit(Some(value));
    });

    let trigger_pointer_down_pos = use_state_eq(|| None);
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

    let content_id = use_id(None);
    let context_value = use_memo(
        (
            props.disabled,
            props.required,
            direction,
            open,
            on_open_change,
            value,
            on_value_change,
            trigger_pointer_down_pos,
        ),
        |(
            disabled,
            required,
            direction,
            open,
            on_open_change,
            value,
            on_value_change,
            trigger_pointer_down_pos,
        )| {
            SelectContextValue {
                trigger_ref,
                value_node_ref,
                content_id,
                value: value.clone(),
                on_value_change: on_value_change.clone(),
                open: *open,
                required: *required,
                on_open_change: on_open_change.clone(),
                dir: *direction,
                trigger_pointer_down_pos: trigger_pointer_down_pos.clone(),
                disabled: *disabled,
            }
        },
    );

    let native_options_context_value = use_memo((), move |_| SelectNativeOptionsContextValue {
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
    });

    html! {
        <Popper>
            <ContextProvider<SelectContextValue> context={(*context_value).clone()}>
                // TODO: CollectionProvider
                <ContextProvider<SelectNativeOptionsContextValue> context={(*native_options_context_value).clone()}>
                    {props.children.clone()}
                </ContextProvider<SelectNativeOptionsContextValue>>

                if *is_form_control {
                    // TODO: BubbleSelect
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
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectTrigger(props: &SelectTriggerProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let is_disabled = context.disabled.unwrap_or(props.disabled);
    let composed_refs =
        use_composed_refs(vec![props.node_ref.clone(), context.trigger_ref.clone()]);
    let pointer_type = use_state_eq(|| "touch".to_string());

    let handle_open = use_callback(
        (context.clone(), is_disabled),
        |event_page_coords: Option<(i32, i32)>, (context, is_disabled)| {
            if !is_disabled {
                context.on_open_change.emit(true);
                // Reset typeahead when we open.
                // TODO: reset_typeahead();
            }

            if let Some(event_page_coords) = event_page_coords {
                context
                    .trigger_pointer_down_pos
                    .set(Some(event_page_coords));
            }
        },
    );

    let attrs = use_memo(
        (
            props.attrs.clone(),
            props.on_click.clone(),
            props.on_pointer_down.clone(),
            props.on_key_down.clone(),
        ),
        move |(attrs, on_click, on_pointer_down, on_key_down)| {
            attrs
                .clone()
                .merge(attrs! {
                    type="button"
                    role="combobox"
                    aria-controls={context.content_id}
                    aria-expanded={match context.open {
                        true => "true",
                        false => "false"
                    }}
                    aria-required={context.required.map(|required| match required {
                        true => "true",
                        false => "false"
                    })}
                    aria-autocomplete="none"
                    dir={context.dir.to_string()}
                    data-state={match context.open {
                        true => "open",
                        false => "closed"
                    }}
                    disabled={is_disabled}
                    data-disabled={is_disabled.then_some("")}
                    data-placeholder={should_show_placeholder(context.value).then_some("")}
                    // Enable compatibility with native label or custom `Label` "click" for Safari:
                    onclick={compose_callbacks(Some(on_click.clone()), Some(Callback::from({
                        let pointer_type = pointer_type.clone();
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
                            if *pointer_type != "mouse" {
                                handle_open.emit(Some((event.page_x(), event.page_y())));
                            }
                    }})), None)}
                    onpointerdown={compose_callbacks(Some(on_pointer_down.clone()), Some(Callback::from({
                        let handle_open = handle_open.clone();

                        move |event: PointerEvent| {
                            pointer_type.set(event.pointer_type());

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
                    })), None)}
                    onkeydown={compose_callbacks(Some(on_key_down.clone()), Some(Callback::from(move |event: KeyboardEvent| {
                        // TODO: typeahead

                        if OPEN_KEYS.contains(&event.key().as_str()) {
                            handle_open.emit(None);
                            event.prevent_default();
                        }
                    })), None)}
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        <PopperAnchor as_child=true>
            <Primitive
                element="button"
                as_child={props.as_child}
                node_ref={composed_refs}
                attrs={(*attrs).clone()}
            >
                {props.children.clone()}
            </Primitive>
        </PopperAnchor>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectValueProps {
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectValue(props: &SelectValueProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let composed_refs = use_composed_refs(vec![props.node_ref.clone(), context.value_node_ref]);

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // We don't want events from the portalled `SelectValue` children to bubble through the item they came from.
                style="pointer-events: none;"
            })
            .expect("Attributes should be merged.")
    });

    // TODO: value node has children?

    html! {
        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={composed_refs}
            attrs={(*attrs).clone()}
        >
            if should_show_placeholder(context.value) {
                {props.placeholder.clone()}
            } else {
                {props.children.clone()}
            }
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectIconProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectIcon(props: &SelectIconProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {match &props.children {
                VNode::VList(list) if list.is_empty() => html!{"▼"},
                children => children.clone(),
            }}
        </Primitive>
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
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectContent(props: &SelectContentProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");

    html! {
        if context.open {
            <SelectContentImpl position={props.position}>
                {props.children.clone()}
            </SelectContentImpl>
        } else {
            // TODO: SelectContentProvider, CollectionSlot
        }
    }
}

const CONTENT_MARGIN: f64 = 10.0;

#[derive(Clone, PartialEq)]
struct SelectContentContextValue {
    content_ref: NodeRef,
    viewport_ref: NodeRef,
    item_ref_callback: Callback<(web_sys::Node, String, bool)>, // TODO
}

#[derive(PartialEq, Properties)]
struct SelectContentImplProps {
    // TODO
    #[prop_or(Position::ItemAligned)]
    pub position: Position,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectContentImpl(props: &SelectContentImplProps) -> Html {
    let content_ref = use_node_ref();
    let viewport_ref = use_node_ref();
    let composed_refs = use_composed_refs(vec![props.node_ref.clone(), content_ref.clone()]);

    let content_context_value = use_memo((), |_| SelectContentContextValue {
        content_ref,
        viewport_ref,
        item_ref_callback: Callback::from(|_| {}),
    });

    html! {
        <ContextProvider<SelectContentContextValue> context={(*content_context_value).clone()}>
            // TODO: RemoveScrol, FocusScope, DismissableLayer

            if props.position == Position::Popper {
                <SelectPopperPosition
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={props.attrs.clone()}
                >
                    {props.children.clone()}
                </SelectPopperPosition>
            } else {
                <SelectItemAlignedPosition
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={props.attrs.clone()}
                >
                    {props.children.clone()}
                </SelectItemAlignedPosition>
            }
        </ContextProvider<SelectContentContextValue>>
    }
}

#[derive(PartialEq, Properties)]
struct SelectItemAlignedPositionProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectItemAlignedPosition(props: &SelectItemAlignedPositionProps) -> Html {
    let _context = use_context::<SelectContextValue>().expect("Select context required.");
    let _content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let content_wrapper_ref = use_node_ref();
    let content_ref = use_node_ref();
    let composed_refs = use_composed_refs(vec![props.node_ref.clone(), content_ref]);
    // TODO

    let content_z_index: UseStateHandle<Option<String>> = use_state_eq(|| None);

    let viewport_context_value = use_memo((), |_| SelectViewportContextValue {
        content_wrapper_ref: content_wrapper_ref.clone(),
    });

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // When we get the height of the content, it includes borders. If we were to set
                // the height without having `box-sizing: border-box` it would be too big.

                // We need to ensure the content doesn't get taller than the wrapper.
                style="box-sizing: border-box; max-height: 100%;"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <ContextProvider<SelectViewportContextValue> context={(*viewport_context_value).clone()}>
            <div
                ref={content_wrapper_ref}
                style={format!("display: flex; flex-direction: column; position: fixed;{}", content_z_index.as_ref().map(|content_z_index| format!("z-index: {content_z_index};")).unwrap_or_default())}
            >
                <Primitive
                    element="span"
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={(*attrs).clone()}
                >
                    {props.children.clone()}
                </Primitive>
            </div>
        </ContextProvider<SelectViewportContextValue>>
    }
}

#[derive(PartialEq, Properties)]
struct SelectPopperPositionProps {
    // TODO
    #[prop_or(Align::Start)]
    pub align: Align,
    #[prop_or(Padding::All(CONTENT_MARGIN))]
    pub collision_padding: Padding,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectPopperPosition(props: &SelectPopperPositionProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO: merge with style attr if present

                // Ensure border-box for Floating UI calculations.
                // Re-namespace exposed content custom properties.
                style="\
                    box-sizing: border-box;\
                    --radix-select-content-transform-origin: var(--radix-popper-transform-origin);\
                    --radix-select-content-available-width: var(--radix-popper-available-width);\
                    --radix-select-content-available-height: var(--radix-popper-available-height);\
                    --radix-select-trigger-width: var(--radix-popper-anchor-width);\
                    --radix-select-trigger-height: var(--radix-popper-anchor-height);\
                "
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <PopperContent
            // TODO: other PopperContent props
            align={props.align}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </PopperContent>
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
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectViewport(props: &SelectViewportProps) -> Html {
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let composed_refs =
        use_composed_refs(vec![props.node_ref.clone(), content_context.viewport_ref]);

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                data-radix-select-viewport=""
                role="presentation"
                // TODO: merge with style attr if present
                // We use position: 'relative' here on the `viewport` so that when we call `selected_item.offset_top` in calculations,
                // the offset is relative to the viewport (independent of the ScrollUpButton).
                style="position: relative; flex: 1; overflow: auto;"
                // TODO: onscroll
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <>
            // Hide scrollbars cross-browser and enable momentum scroll for touch devices.
            <style nonce={props.nonce.clone()}>
                {"[data-radix-select-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-select-viewport]::-webkit-scrollbar{display:none;}"}
            </style>

            // TODO: CollectionSlot
            <Primitive
                element="div"
                as_child={props.as_child}
                node_ref={composed_refs}
                attrs={(*attrs).clone()}
            >
                {props.children.clone()}
            </Primitive>
        </>
    }
}

#[derive(Clone, PartialEq)]
struct SelectGroupContextValue {
    id: String,
}

#[derive(PartialEq, Properties)]
pub struct SelectGroupProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectGroup(props: &SelectGroupProps) -> Html {
    let group_id = use_id(None);

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                role="group"
                aria-labelledby={group_id.clone()}
            })
            .expect("Attributes should be merged.")
    });

    let context_value = use_memo(group_id, |group_id| SelectGroupContextValue {
        id: group_id.clone(),
    });

    html! {
        <ContextProvider<SelectGroupContextValue> context={(*context_value).clone()}>
            <Primitive
                element="div"
                as_child={props.as_child}
                node_ref={props.node_ref.clone()}
                attrs={(*attrs).clone()}
            >
                {props.children.clone()}
            </Primitive>
        </ContextProvider<SelectGroupContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectLabelProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectLabel(props: &SelectLabelProps) -> Html {
    let group_context =
        use_context::<SelectGroupContextValue>().expect("Select group context required.");

    let attrs = use_memo(
        (props.attrs.clone(), group_context),
        |(attrs, group_context)| {
            attrs
                .clone()
                .merge(attrs! {
                    id={group_context.id.clone()}
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        <Primitive
            element="div"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(Clone, PartialEq)]
struct SelectItemContextValue {
    value: String,
    disabled: bool,
    text_id: String,
    is_selected: bool,
    on_item_text_change: Callback<web_sys::Node>,
}

#[derive(PartialEq, Properties)]
pub struct SelectItemProps {
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub text_value: String,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItem(props: &SelectItemProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let _text_value = use_state_eq(|| props.text_value.clone());
    let is_focused = use_state_eq(|| false);
    let item_ref = use_node_ref();
    let composed_refs = use_composed_refs(vec![props.node_ref.clone(), item_ref.clone()]);
    let is_selected = context.value.is_some_and(|value| value == props.value);
    let text_id = use_id(None);

    let item_ref_callback = use_callback(
        (
            content_context.item_ref_callback,
            props.value.clone(),
            props.disabled,
        ),
        |node: web_sys::Node, (item_ref_callback, value, disabled)| {
            item_ref_callback.emit((node, value.clone(), *disabled))
        },
    );
    use_effect_with(item_ref, move |item_ref| {
        if let Some(node) = item_ref.get() {
            item_ref_callback.emit(node);
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
            on_item_text_change: Callback::from(|_| {}),
        },
    );

    let attrs = use_memo(
        (
            props.attrs.clone(),
            props.disabled,
            text_id,
            is_focused,
            is_selected,
        ),
        |(attrs, disabled, text_id, is_focused, is_selected)| {
            attrs
                .clone()
                .merge(attrs! {
                    role="option"
                    aria-labelledby={text_id.clone()}
                    data-highlighted={is_focused.then_some("")}
                    // `is_focused` caveat fixes stuttering in VoiceOver.
                    aria-selected={(*is_selected && **is_focused).then_some("true")}
                    data-state={if *is_selected { "checked" } else { "unchecked "}}
                    aria-disabled={disabled.then_some("true")}
                    data-disabled={disabled.then_some("")}
                    tab-index={(!disabled).then_some("-1")}
                    // TODO: events
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        <ContextProvider<SelectItemContextValue> context={(*item_context_value).clone()}>
            // TODO: CollectionItemSlot
            <Primitive
                element="div"
                as_child={props.as_child}
                node_ref={composed_refs}
                attrs={(*attrs).clone()}
            >
                {props.children.clone()}
            </Primitive>
        </ContextProvider<SelectItemContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectItemTextProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItemText(props: &SelectItemTextProps) -> Html {
    let _context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let item_context =
        use_context::<SelectItemContextValue>().expect("Select item context required.");
    let _native_options_context = use_context::<SelectNativeOptionsContextValue>()
        .expect("Select native options context required.");
    let item_text_node_ref = use_node_ref();
    let composed_refs = use_composed_refs(vec![props.node_ref.clone(), item_text_node_ref.clone()]);

    let item_ref_callback = use_callback(
        (content_context.item_ref_callback, item_context.clone()),
        |node: web_sys::Node, (item_ref_callback, item_context)| {
            item_context.on_item_text_change.emit(node.clone());
            item_ref_callback.emit((node, item_context.value.clone(), item_context.disabled));
        },
    );
    use_effect_with(item_text_node_ref, move |item_text_node_ref| {
        if let Some(node) = item_text_node_ref.get() {
            item_ref_callback.emit(node);
        }
    });

    let attrs = use_memo(
        (props.attrs.clone(), item_context),
        |(attrs, item_context)| {
            attrs
                .clone()
                .merge(attrs! {
                    id={item_context.text_id.clone()}
                    // TODO
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        // TODO

        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={composed_refs}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectItemIndicatorProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItemIndicator(props: &SelectItemIndicatorProps) -> Html {
    // let item_context = use_context::<SelectItemContextValue>().expect("Select item context required.");

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        // TODO

        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectScrollUpButtonProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectScrollUpButton(props: &SelectScrollUpButtonProps) -> Html {
    // TODO
    html! {
        <SelectScrollButtonImpl
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={props.attrs.clone()}
        >
            {props.children.clone()}
        </SelectScrollButtonImpl>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectScrollDownButtonProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectScrollDownButton(props: &SelectScrollDownButtonProps) -> Html {
    // TODO
    html! {
        <SelectScrollButtonImpl
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={props.attrs.clone()}
        >
            {props.children.clone()}
        </SelectScrollButtonImpl>
    }
}

#[derive(PartialEq, Properties)]
struct SelectScrollButtonImplProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectScrollButtonImpl(props: &SelectScrollButtonImplProps) -> Html {
    // let content_context = use_context::<SelectContentContextValue>().expect("Select content context required.");

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="div"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectSeparatorProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectSeparator(props: &SelectSeparatorProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="div"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectArrowProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectArrow(props: &SelectArrowProps) -> Html {
    // TODO
    html! {
        <PopperArrow
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={props.attrs.clone()}
        >
            {props.children.clone()}
        </PopperArrow>
    }
}
fn should_show_placeholder(value: Option<String>) -> bool {
    value.is_none() || value.is_some_and(|value| value.is_empty())
}
