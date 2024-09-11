use radix_yew_primitive::Primitive;
use yew::prelude::*;
use yew_attrs::{attrs, Attrs};

#[derive(PartialEq, Properties)]
pub struct ArrowProps {
    #[prop_or(10.0)]
    pub width: f64,
    #[prop_or(5.0)]
    pub height: f64,
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
pub fn Arrow(props: &ArrowProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                width={props.width.to_string()}
                height={props.height.to_string()}
                viewBox="0 0 30 10"
                preserveAspectRatio="none"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="svg"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            if props.as_child {
                {props.children.clone()}
            } else {
                <polygon points="0,0 30,0 15,10" />
            }
        </Primitive>
    }
}
