use radix_yew_themes::{
    DataList, DataListItem, DataListLabel, DataListValue, Flex, FlexDirection, Link,
};
use yew::prelude::*;

#[function_component]
pub fn DataListSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=6>
            <DataList size=1>
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

            <DataList size=2>
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

            <DataList size=3>
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
        </Flex>
    }
}
