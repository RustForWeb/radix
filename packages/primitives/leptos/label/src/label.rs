use leptos::{ev::MouseEvent, prelude::*};

pub struct UseLabelProps {
    on_mouse_down: Option<Callback<MouseEvent>>,
}

pub struct UseLabelReturn {
    on_mouse_down: Callback<MouseEvent>,
}

pub fn use_label(props: UseLabelProps) -> UseLabelReturn {
    UseLabelReturn {
        on_mouse_down: Callback::new(move |event: MouseEvent| {
            // Only prevent text selection if clicking inside the label itself
            let target = event_target::<web_sys::Element>(&event);
            if target
                .closest("button, input, select, textarea")
                .expect("Element should be able to query closest.")
                .is_some()
            {
                return;
            }

            if let Some(on_mouse_down) = props.on_mouse_down {
                on_mouse_down.run(event.clone());
            }

            // Prevent text selection when double clicking label
            if !event.default_prevented() && event.detail() > 1 {
                event.prevent_default();
            }
        }),
    }
}

#[component]
pub fn Label(
    #[prop(into, optional)] on_mouse_down: Option<Callback<MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let UseLabelReturn { on_mouse_down } = use_label(UseLabelProps { on_mouse_down });

    view! {
        <label on:mousedown=move |event| on_mouse_down.run(event)>
            {children.map(|children| children())}
        </label>
    }
}
