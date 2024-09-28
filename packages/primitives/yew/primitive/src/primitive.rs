use radix_yew_slot::Slot;
use yew::prelude::*;
use yew_attrs::Attrs;

#[derive(PartialEq, Properties)]
pub struct PrimitiveProps {
    pub element: String,
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
pub fn Primitive(props: &PrimitiveProps) -> Html {
    if props.as_child {
        html! {
            <Slot node_ref={props.node_ref.clone()} attrs={props.attrs.clone()}>
                {props.children.clone()}
            </Slot>
        }
    } else {
        props
            .attrs
            .clone()
            .new_vtag(
                &props.element,
                props.node_ref.clone(),
                Default::default(),
                props.children.clone(),
            )
            .into()
    }
}

pub fn compose_callbacks<E: Clone + Into<Event> + 'static>(
    original_event_handler: Option<Callback<E>>,
    our_event_handler: Option<Callback<E>>,
    check_for_default_prevented: Option<bool>,
) -> Callback<E> {
    let check_for_default_prevented = check_for_default_prevented.unwrap_or(true);

    Callback::from(move |event: E| {
        if let Some(original_event_handler) = &original_event_handler {
            original_event_handler.emit(event.clone());
        }

        if !check_for_default_prevented || !event.clone().into().default_prevented() {
            if let Some(our_event_handler) = &our_event_handler {
                our_event_handler.emit(event);
            }
        }
    })
}
