use radix_yew_themes::{Box, Section};
use yew::prelude::*;

use crate::decorative_box::{DecorativeBox, DecorativeBoxChildProps};

#[function_component]
pub fn SectionExample() -> Html {
    html! {
        <Box py=8 style={[("background", "var(--gray-a2)"), ("border-radius", "var(--radius-3)")]}>
            <DecorativeBox
                as_child={Callback::from(|DecorativeBoxChildProps {height, style}| html! {
                    <Section size=2 height={height} style={style} />
                })}
            />
        </Box>
    }
}
