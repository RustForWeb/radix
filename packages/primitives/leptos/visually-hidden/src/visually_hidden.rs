use leptos::prelude::*;
use leptos_style::Style;

pub struct UseVisuallyHiddenProps {
    style: Style,
}

pub struct UseVisuallyHiddenAttrs {
    style: Style,
}

pub fn use_visually_hidden(props: UseVisuallyHiddenProps) -> UseVisuallyHiddenAttrs {
    UseVisuallyHiddenAttrs {
        style: props.style.with_defaults([
            // See: https://github.com/twbs/bootstrap/blob/master/scss/mixins/_screen-reader.scss
            ("position", "absolute"),
            ("border", "0px"),
            ("width", "1px"),
            ("height", "1px"),
            ("padding", "0px"),
            ("margin", "-1px"),
            ("overflow", "hidden"),
            ("clip", "rect(0, 0, 0, 0)"),
            ("white-space", "nowrap"),
            ("word-wrap", "normal"),
        ]),
    }
}

#[component]
pub fn VisuallyHidden(
    #[prop(into, optional)] style: Style,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let UseVisuallyHiddenAttrs { style } = use_visually_hidden(UseVisuallyHiddenProps { style });

    view! {
        <span style=style>
            {children.map(|children| children())}
        </span>
    }
}

#[component]
pub fn VisuallyHiddenAsChild<R, RV>(
    #[prop(into, optional)] style: Style,
    render: R,
) -> impl IntoView
where
    R: Fn(UseVisuallyHiddenAttrs) -> RV,
    RV: IntoView,
{
    let attrs = use_visually_hidden(UseVisuallyHiddenProps { style });

    render(attrs)
}
