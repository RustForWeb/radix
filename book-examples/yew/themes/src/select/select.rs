use radix_yew_themes::{
    Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger,
};
use yew::prelude::*;

#[function_component]
pub fn SelectExample() -> Html {
    html! {
        <Select default_value="apple">
            <SelectTrigger />
            <SelectContent>
                <SelectGroup>
                    <SelectLabel>{"Fruits"}</SelectLabel>
                    <SelectItem value="orange">{"Orange"}</SelectItem>
                    <SelectItem value="apple">{"Apple"}</SelectItem>
                    <SelectItem value="grape" disabled=true>
                        {"Grape"}
                    </SelectItem>
                </SelectGroup>
                <SelectSeparator />
                <SelectGroup>
                    <SelectLabel>{"Vegetables"}</SelectLabel>
                    <SelectItem value="carrot">{"Carrot"}</SelectItem>
                    <SelectItem value="potato">{"Potato"}</SelectItem>
                </SelectGroup>
            </SelectContent>
        </Select>
    }
}
