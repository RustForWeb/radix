use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use radix_yew_compose_refs::use_composed_refs;
use radix_yew_popper::{Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_yew_primitive::Primitive;
use yew::{prelude::*, virtual_dom::VNode};
use yew_attrs::{attrs, Attrs};

#[derive(Clone, Copy, Debug, PartialEq)]
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
    // dir: Direction,
    // TODO: trigger pointer down pos
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
    pub default_open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    // #[prop_or_default]
    // pub dir: Option<Direction>,
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
    // let direction = use_direction(props.dir);
    // TODO: controllable state for open and value
    let open = use_state_eq(|| false);
    let on_open_change = Callback::from({
        let open = open.clone();

        move |new_value: bool| open.set(new_value)
    });
    let value: UseStateHandle<Option<String>> = use_state_eq(|| None);
    let on_value_change = Callback::from({
        let value = value.clone();

        move |new_value: String| value.set(Some(new_value))
    });

    let is_form_control = trigger_ref
        .cast::<web_sys::Element>()
        .map(|trigger| trigger.closest("form").ok().flatten().is_some())
        .unwrap_or(true);
    let native_options_set = use_state_eq(HashSet::<NativeOption>::new);

    let context_value = use_memo(
        (props.disabled, props.required, open, value),
        |(disabled, required, open, value)| SelectContextValue {
            trigger_ref,
            value_node_ref,
            // TODO
            content_id: "".into(),
            value: (**value).clone(),
            on_value_change,
            open: **open,
            required: *required,
            on_open_change,
            disabled: *disabled,
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

                if is_form_control {
                    // TODO: BubbleSelect
                }
            </ContextProvider<SelectContextValue>>
        </Popper>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectTriggerProps {
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
    let composed_refs = use_composed_refs(vec![props.node_ref.clone(), context.trigger_ref]);

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO
            })
            .expect("Attributes should be merged.")
    });

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
                aria-hidden=""
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
    // TODO: use_id()
    let group_id = "".to_string();

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
    // TODO: use_id()
    let text_id = "".to_string();

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
                aria-hidden=""
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
                aria-hidden=""
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
                aria-hidden=""
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
