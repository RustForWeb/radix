use radix_yew_icons::PlusIcon;
use radix_yew_themes::{IconButton, Radius, Tooltip, TooltipChildProps};
use yew::prelude::*;

#[function_component]
pub fn TooltipExample() -> Html {
    html! {
        <Tooltip
            content="Add to library"

            as_child={Callback::from(|TooltipChildProps {
                node_ref,
                attributes,

                aria_describedby,
                data_state,
                class,
                id,
                style,

                // onblur,
                // onclick,
                // onfocus,
                // onpointerdown,
                // onpointerleave,
                // onpointermove,
                ..
            }| html! {
                <IconButton
                    radius={Radius::Full}

                    class={class}
                    id={id}
                    style={style}

                    // on_blur={onblur}
                    // on_click={onclick}
                    // on_focus={onfocus}
                    // on_pointer_down={onpointerdown}
                    // on_pointer_leave={onpointerleave}
                    // on_pointer_move={onpointermove}

                    node_ref={node_ref}
                    attributes={attributes.with_defaults([
                        ("aria-describedby", aria_describedby),
                        ("data-state", Some(data_state)),
                    ])}
                >
                    <PlusIcon />
                </IconButton>
            })}
        />
    }
}
