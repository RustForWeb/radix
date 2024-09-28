use radix_yew_primitive::Primitive;
use yew::prelude::*;
use yew_attrs::{attrs, Attrs};

#[derive(PartialEq, Properties)]
pub struct VisuallyHiddenProps {
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
pub fn VisuallyHidden(props: &VisuallyHiddenProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO: merge style with props.attrs.style

                // See https://github.com/twbs/bootstrap/blob/master/scss/mixins/_screen-reader.scss.
                style="position: absolute; border: 0px; width: 1px; height: 1px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; word-wrap: normal;"
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
            {props.children.clone()}
        </Primitive>
    }
}
