use radix_yew_themes::{
    AccentColor, DataList, DataListItem, DataListLabel, DataListOrientation, DataListValue,
};
use yew::prelude::*;

#[function_component]
pub fn DataListColorExample() -> Html {
    html! {
        <DataList orientation={DataListOrientation::Vertical}>
            <DataListItem>
                <DataListLabel color={AccentColor::Indigo}>{"Name"}</DataListLabel>
                <DataListValue>{"Indigo"}</DataListValue>
            </DataListItem>
            <DataListItem>
                <DataListLabel color={AccentColor::Cyan}>{"Name"}</DataListLabel>
                <DataListValue>{"Cyan"}</DataListValue>
            </DataListItem>
            <DataListItem>
                <DataListLabel color={AccentColor::Orange}>{"Name"}</DataListLabel>
                <DataListValue>{"Orange"}</DataListValue>
            </DataListItem>
            <DataListItem>
                <DataListLabel color={AccentColor::Crimson}>{"Name"}</DataListLabel>
                <DataListValue>{"Crimson"}</DataListValue>
            </DataListItem>
        </DataList>
    }
}
