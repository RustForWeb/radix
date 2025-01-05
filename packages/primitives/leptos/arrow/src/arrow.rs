use leptos::prelude::*;

pub struct UseArrowProps {
    width: MaybeProp<f64>,
    height: MaybeProp<f64>,
}

pub struct UseArrowReturn {
    width: Signal<String>,
    height: Signal<String>,
    view_box: String,
    preserve_aspect_ratio: String,
}

pub fn use_arrow(props: UseArrowProps) -> UseArrowReturn {
    let width = Signal::derive(move || props.width.get().unwrap_or(10.0).to_string());
    let height = Signal::derive(move || props.height.get().unwrap_or(5.0).to_string());

    UseArrowReturn {
        width,
        height,
        view_box: "0 0 30 10".to_owned(),
        preserve_aspect_ratio: "none".to_owned(),
    }
}

#[component]
pub fn Arrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let UseArrowReturn {
        width,
        height,
        view_box,
        preserve_aspect_ratio,
    } = use_arrow(UseArrowProps { width, height });

    view! {
        <svg
            width=width
            height=height
            viewBox=view_box
            preserveAspectRatio=preserve_aspect_ratio
        >
            {children.map(|children| children()).unwrap_or_else(|| view! {
                <polygon points="0,0 30,0 15,10" />
            }.into_any())}
        </svg>
    }
}
