use radix_yew_themes::{
    AccentColor, DataList, DataListItem, DataListLabel, DataListOrientation, DataListValue, Flex,
};
use yew::prelude::*;

#[function_component]
pub fn DataListHighContrastExample() -> Html {
    html! {
        <Flex gap=9>
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

            <DataList orientation={DataListOrientation::Vertical}>
                <DataListItem>
                    <DataListLabel color={AccentColor::Indigo} high_contrast=true>
                        {"Name"}
                    </DataListLabel>
                    <DataListValue>{"Indigo"}</DataListValue>
                </DataListItem>
                <DataListItem>
                    <DataListLabel color={AccentColor::Cyan} high_contrast=true>
                        {"Name"}
                    </DataListLabel>
                    <DataListValue>{"Cyan"}</DataListValue>
                </DataListItem>
                <DataListItem>
                    <DataListLabel color={AccentColor::Orange} high_contrast=true>
                        {"Name"}
                    </DataListLabel>
                    <DataListValue>{"Orange"}</DataListValue>
                </DataListItem>
                <DataListItem>
                    <DataListLabel color={AccentColor::Crimson} high_contrast=true>
                        {"Name"}
                    </DataListLabel>
                    <DataListValue>{"Crimson"}</DataListValue>
                </DataListItem>
            </DataList>
        </Flex>
    }
}
