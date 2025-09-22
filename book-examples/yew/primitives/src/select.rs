// TODO: Remove
#![allow(dead_code)]

use radix_yew_icons::{CheckIcon, ChevronDownIcon, ChevronUpIcon};
use radix_yew_select::*;
use yew::prelude::*;

#[function_component]
pub fn SelectDemo() -> Html {
    html! {
        <Select>
            <SelectTrigger
                class="inline-flex h-[35px] items-center justify-center gap-[5px] rounded bg-white px-[15px] text-[13px] leading-none text-violet11 shadow-[0_2px_10px] shadow-black/10 outline-none hover:bg-mauve3 focus:shadow-[0_0_0_2px] focus:shadow-black data-[placeholder]:text-violet9"
                // TODO
                // aria-label="Food"
            >
            <SelectValue placeholder="Select a fruit…" />
            <SelectIcon class="text-violet11">
                <ChevronDownIcon />
            </SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent class="overflow-hidden rounded-md bg-white shadow-[0px_10px_38px_-10px_rgba(22,_23,_24,_0.35),0px_10px_20px_-15px_rgba(22,_23,_24,_0.2)]">
                    <SelectScrollUpButton class="flex h-[25px] cursor-default items-center justify-center bg-white text-violet11">
                        <ChevronUpIcon />
                    </SelectScrollUpButton>
                    <SelectViewport class="p-[5px]">
                        <SelectGroup>
                            <SelectLabel class="px-[25px] text-xs leading-[25px] text-mauve11">
                                {"Fruits"}
                            </SelectLabel>
                            <SelectDemoItem value="apple">{"Apple"}</SelectDemoItem>
                            <SelectDemoItem value="banana">{"Banana"}</SelectDemoItem>
                            <SelectDemoItem value="blueberry">{"Blueberry"}</SelectDemoItem>
                            <SelectDemoItem value="grapes">{"Grapes"}</SelectDemoItem>
                            <SelectDemoItem value="pineapple">{"Pineapple"}</SelectDemoItem>
                        </SelectGroup>

                        <SelectSeparator class="m-[5px] h-px bg-violet6" />

                        <SelectGroup>
                            <SelectLabel class="px-[25px] text-xs leading-[25px] text-mauve11">
                                {"Vegetables"}
                            </SelectLabel>
                            <SelectDemoItem value="aubergine">{"Aubergine"}</SelectDemoItem>
                            <SelectDemoItem value="broccoli">{"Broccoli"}</SelectDemoItem>
                            <SelectDemoItem value="carrot" disabled=true>
                                {"Carrot"}
                            </SelectDemoItem>
                            <SelectDemoItem value="courgette">{"Courgette"}</SelectDemoItem>
                            <SelectDemoItem value="leek">{"Leek"}</SelectDemoItem>
                        </SelectGroup>

                        <SelectSeparator class="m-[5px] h-px bg-violet6" />

                        <SelectGroup>
                            <SelectLabel class="px-[25px] text-xs leading-[25px] text-mauve11">
                                {"Meat"}
                            </SelectLabel>
                            <SelectDemoItem value="beef">{"Beef"}</SelectDemoItem>
                            <SelectDemoItem value="chicken">{"Chicken"}</SelectDemoItem>
                            <SelectDemoItem value="lamb">{"Lamb"}</SelectDemoItem>
                            <SelectDemoItem value="pork">{"Pork"}</SelectDemoItem>
                        </SelectGroup>
                    </SelectViewport>
                    <SelectScrollDownButton class="flex h-[25px] cursor-default items-center justify-center bg-white text-violet11">
                        <ChevronDownIcon />
                    </SelectScrollDownButton>
                </SelectContent>
            </SelectPortal>
        </Select>
    }
}

#[derive(PartialEq, Properties)]
struct SelectDemoItemProps {
    pub value: String,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectDemoItem(props: &SelectItemProps) -> Html {
    html! {
        <SelectItem
            class="relative flex h-[25px] select-none items-center rounded-[3px] pl-[25px] pr-[35px] text-[13px] leading-none text-violet11 data-[disabled]:pointer-events-none data-[highlighted]:bg-violet9 data-[disabled]:text-mauve8 data-[highlighted]:text-violet1 data-[highlighted]:outline-none"
            value={props.value.clone()}
        >
            <SelectItemText>{props.children.clone()}</SelectItemText>
            <SelectItemIndicator class="absolute left-0 inline-flex w-[25px] items-center justify-center">
                <CheckIcon />
            </SelectItemIndicator>
        </SelectItem>
    }
}
