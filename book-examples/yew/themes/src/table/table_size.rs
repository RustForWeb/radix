use radix_yew_themes::{
    Flex, FlexDirection, Table, TableBody, TableCell, TableColumnHeaderCell, TableHeader, TableRow,
    TableRowHeaderCell,
};
use yew::prelude::*;

#[function_component]
pub fn TableSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=5 max_width="350px">
            <Table size=1>
                <TableHeader>
                    <TableRow>
                        <TableColumnHeaderCell>{"Full name"}</TableColumnHeaderCell>
                        <TableColumnHeaderCell>{"Email"}</TableColumnHeaderCell>
                    </TableRow>
                </TableHeader>

                <TableBody>
                    <TableRow>
                        <TableRowHeaderCell>{"Danilo Sousa"}</TableRowHeaderCell>
                        <TableCell>{"danilo@example.com"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableRowHeaderCell>{"Zahra Ambessa"}</TableRowHeaderCell>
                        <TableCell>{"zahra@example.com"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>

            <Table size=2>
                <TableHeader>
                    <TableRow>
                        <TableColumnHeaderCell>{"Full name"}</TableColumnHeaderCell>
                        <TableColumnHeaderCell>{"Email"}</TableColumnHeaderCell>
                    </TableRow>
                </TableHeader>

                <TableBody>
                    <TableRow>
                        <TableRowHeaderCell>{"Danilo Sousa"}</TableRowHeaderCell>
                        <TableCell>{"danilo@example.com"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableRowHeaderCell>{"Zahra Ambessa"}</TableRowHeaderCell>
                        <TableCell>{"zahra@example.com"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>

            <Table size=3>
                <TableHeader>
                    <TableRow>
                        <TableColumnHeaderCell>{"Full name"}</TableColumnHeaderCell>
                        <TableColumnHeaderCell>{"Email"}</TableColumnHeaderCell>
                    </TableRow>
                </TableHeader>

                <TableBody>
                    <TableRow>
                        <TableRowHeaderCell>{"Danilo Sousa"}</TableRowHeaderCell>
                        <TableCell>{"danilo@example.com"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableRowHeaderCell>{"Zahra Ambessa"}</TableRowHeaderCell>
                        <TableCell>{"zahra@example.com"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </Flex>
    }
}
