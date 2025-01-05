use leptos::prelude::*;
use leptos_style::Style;

pub struct UseAspectRatioProps {
    style: Style,
}

pub struct UseAspectRatioAttrs {
    style: Style,
}

pub fn use_aspect_ratio(props: UseAspectRatioProps) -> UseAspectRatioAttrs {
    UseAspectRatioAttrs {
        style: props.style.with_defaults([
            // Ensures children expand in ratio.
            ("position", "absolute"),
            ("top", "0px"),
            ("right", "0px"),
            ("bottom", "0px"),
            ("left", "0px"),
        ]),
    }
}

#[component]
pub fn BaseAspectRatio(
    #[prop(into, optional)] ratio: MaybeProp<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ratio = Signal::derive(move || ratio.get().unwrap_or(1.0));

    view! {
        <div
            // Ensures inner element is contained.
            style:position="relative"
            // Ensures padding bottom trick maths works.
            style:width="100%"
            style:padding-bottom=move || format!("{}%", 100.0 / ratio.get())
            data-radix-aspect-ratio-wrapper=""
        >
            {children.map(|children| children())}
        </div>
    }
}

#[component]
pub fn AspectRatio(
    #[prop(into, optional)] ratio: MaybeProp<f64>,
    #[prop(into, optional)] style: Style,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let attrs = use_aspect_ratio(UseAspectRatioProps { style });

    view! {
        <BaseAspectRatio ratio=ratio>
            <div style={attrs.style}>
                {children.map(|children| children())}
            </div>
        </BaseAspectRatio>
    }
}

#[component]
pub fn AspectRatioAsChild<R, RV>(
    #[prop(into, optional)] ratio: MaybeProp<f64>,
    #[prop(into, optional)] style: Style,
    render: R,
) -> impl IntoView
where
    R: Fn(UseAspectRatioAttrs) -> RV + Send + 'static,
    RV: IntoView + 'static,
{
    let attrs = use_aspect_ratio(UseAspectRatioProps { style });

    view! {
        <BaseAspectRatio ratio=ratio>
            {render(attrs)}
        </BaseAspectRatio>
    }
}
