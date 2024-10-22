use std::collections::HashMap;

use radix_yew_themes::Box;
use yew::prelude::*;

#[function_component]
pub fn DecorativeBox() -> Html {
    html! {
        <Box
            height="100%"
            style={HashMap::from([
                ("background-color".into(), "var(--gray-a3)".into()),
                ("background-clip".into(), "padding-box".into()),
                ("border".into(), "1px solid var(--gray-a5)".into()),
                ("border-radius".into(), "var(--radius-1)".into()),
                ("background-image".into(), "url(\"data:image/svg+xml,%3Csvg width='6' height='6' viewBox='0 0 6 6' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='%239C92AC' fill-opacity='0.2' fill-rule='evenodd'%3E%3Cpath d='M5 0h1L0 6V5zM6 5v1H5z'/%3E%3C/g%3E%3C/svg%3E\")".into()),
            ])}
        />
    }
}
