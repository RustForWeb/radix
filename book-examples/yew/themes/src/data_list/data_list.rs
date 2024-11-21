use radix_yew_icons::CopyIcon;
use radix_yew_themes::{
    AccentColor, Badge, BadgeVariant, Code, CodeVariant, DataList, DataListItem, DataListItemAlign,
    DataListLabel, DataListValue, Flex, FlexAlign, IconButton, IconButtonVariant, Link, Radius,
};
use yew::prelude::*;

#[function_component]
pub fn DataListExample() -> Html {
    html! {
        <DataList>
            <DataListItem align={DataListItemAlign::Center}>
                <DataListLabel min_width="88px">{"Status"}</DataListLabel>
                <DataListValue>
                    <Badge color={AccentColor::Jade} variant={BadgeVariant::Soft} radius={Radius::Full}>
                        {"Authorized"}
                    </Badge>
                </DataListValue>
            </DataListItem>
            <DataListItem>
                <DataListLabel min_width="88px">{"ID"}</DataListLabel>
                <DataListValue>
                    <Flex align={FlexAlign::Center} gap=2>
                        <Code variant={CodeVariant::Ghost}>{"u_2J89JSA4GJ"}</Code>
                        <IconButton
                            size=1
                            color={AccentColor::Gray}
                            variant={IconButtonVariant::Ghost}
                            attributes={[
                                ("aria-label", "Copy value")
                            ]}
                        >
                            <CopyIcon />
                        </IconButton>
                    </Flex>
                </DataListValue>
            </DataListItem>
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
