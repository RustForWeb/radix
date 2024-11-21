use radix_yew_themes::{
    Breakpoint, DataList, DataListItem, DataListLabel, DataListOrientation, DataListValue, Link,
    ResponsiveValues,
};
use yew::prelude::*;

#[function_component]
pub fn DataListOrientationExample() -> Html {
    html! {
        <DataList
            orientation={ResponsiveValues::from([
                (Breakpoint::Initial, DataListOrientation::Vertical),
                (Breakpoint::Sm, DataListOrientation::Horizontal),
            ])}
        >
            <DataListItem>
                <DataListLabel min_width="88px">{"Name"}</DataListLabel>
                <DataListValue>{"Vlad Moroz"}</DataListValue>
            </DataListItem>
            <DataListItem>
                <DataListLabel min_width="88px">{"Email"}</DataListLabel>
                <DataListValue>
                    <Link href="mailto:vlad@workos.com">{"vlad@workos.com"}</Link>
                </DataListValue>
            </DataListItem>
            <DataListItem>
                <DataListLabel min_width="88px">{"Company"}</DataListLabel>
                <DataListValue>
                    <Link target="_blank" href="https://workos.com">
                        {"WorkOS"}
                    </Link>
                </DataListValue>
            </DataListItem>
        </DataList>
    }
}
