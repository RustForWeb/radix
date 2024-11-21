use radix_yew_themes::{
    Table, TableBody, TableCell, TableColumnHeaderCell, TableHeader, TableRow, TableRowHeaderCell,
};
use yew::prelude::*;

#[function_component]
pub fn TableExample() -> Html {
    html! {
        <Table>
            <TableHeader>
                <TableRow>
                    <TableColumnHeaderCell>{"Full name"}</TableColumnHeaderCell>
                    <TableColumnHeaderCell>{"Email"}</TableColumnHeaderCell>
                    <TableColumnHeaderCell>{"Group"}</TableColumnHeaderCell>
                </TableRow>
            </TableHeader>

            <TableBody>
                <TableRow>
                    <TableRowHeaderCell>{"Danilo Sousa"}</TableRowHeaderCell>
                    <TableCell>{"danilo@example.com"}</TableCell>
                    <TableCell>{"Developer"}</TableCell>
                </TableRow>

                <TableRow>
                    <TableRowHeaderCell>{"Zahra Ambessa"}</TableRowHeaderCell>
                    <TableCell>{"zahra@example.com"}</TableCell>
                    <TableCell>{"Admin"}</TableCell>
                </TableRow>

                <TableRow>
                    <TableRowHeaderCell>{"Jasper Eriksson"}</TableRowHeaderCell>
                    <TableCell>{"jasper@example.com"}</TableCell>
                    <TableCell>{"Developer"}</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}
