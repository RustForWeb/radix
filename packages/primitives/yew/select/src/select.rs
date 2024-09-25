use std::fmt::{Display, Formatter};

use radix_yew_compose_refs::use_composed_refs;
use radix_yew_popper::{Popper, PopperAnchor, PopperArrow};
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

#[derive(PartialEq, Properties)]
pub struct SelectProps {
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

    html! {
        <Popper>
            <ContextProvider<SelectContextValue> context={(*context_value).clone()}>
                // TODO: CollectionProvider, SelectNativeOptionsProvider
                {props.children.clone()}

                // TODO: BubbleSelect
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
    html! {
        // TODO: SelectContentProvider, CollectionSlot

        <SelectContentImpl position={props.position}>
            {props.children.clone()}
        </SelectContentImpl>
    }
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
    html! {
        {props.children.clone()}
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
    html! {
        {props.children.clone()}
    }
}

#[derive(PartialEq, Properties)]
struct SelectPopperPositionProps {
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
fn SelectPopperPosition(props: &SelectPopperPositionProps) -> Html {
    html! {
        {props.children.clone()}
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectViewportProps {
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
pub fn SelectViewport(props: &SelectViewportProps) -> Html {
    html! {
        {props.children.clone()}
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
    // let context = use_context::<SelectContextValue>().expect("Select context required.");

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO
            })
            .expect("Attributes should be merged.")
    });

    html! {
        // TODO

        <Primitive
            element="option"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
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
    // let context = use_context::<SelectContextValue>().expect("Select context required.");

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO
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
