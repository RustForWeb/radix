use leptos::*;
use radix_leptos_menu::*;
use tailwind_fuse::*;

// TODO: add router and separate pages for each component, similar to Storybook

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1 class="text-xl pb-3">Styled</h1>

        <div class="h-[400px] overflow-y-auto">
            <Styled />
        </div>
    }
}

#[component]
fn Styled() -> impl IntoView {
    let item_class = create_memo(move |_| ItemClass::default().to_class());

    view! {
        <MenuWithAnchor>
            <MenuItem class=item_class>
                Undo
            </MenuItem>
            <MenuItem class=item_class>
                Redo
            </MenuItem>
            <MenuSeparator />
            <MenuItem class=item_class>
                Cut
            </MenuItem>
            <MenuItem class=item_class>
                Copy
            </MenuItem>
            <MenuItem class=item_class>
                Paste
            </MenuItem>
        </MenuWithAnchor>
    }
}

#[component]
fn Submenus() -> impl IntoView {
    view! {}
}

#[component]
fn WithLabels() -> impl IntoView {
    view! {}
}

#[component]
fn Typeahead() -> impl IntoView {
    view! {}
}

#[component]
fn CheckboxItems() -> impl IntoView {
    view! {}
}

#[component]
fn RadioItems() -> impl IntoView {
    view! {}
}

#[component]
fn Animated() -> impl IntoView {
    view! {}
}

#[component]
fn MenuWithAnchor(
    #[prop(into, optional)] open: MaybeProp<bool>,
    children: Children,
) -> impl IntoView {
    let open = Signal::derive(move || open().unwrap_or(true));

    let content_class = create_memo(move |_| ContentClass::default().to_class());

    // TODO: add missing props
    view! {
        <Menu open=open modal=false>
            <MenuAnchor>{""}</MenuAnchor>
            <MenuPortal>
                <MenuContent class=content_class>
                    {children()}
                </MenuContent>
            </MenuPortal>
        </Menu>
    }
}

#[component]
fn Submenu() -> impl IntoView {
    view! {}
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-block box-border min-w-[130px] bg-[#fff] border border-solid border-[#ccc] rounded-[6px] p-[5px] shadow-[0px_5px_10px_0px_rgba(0,0,0,0.1)] font-['apple-system, BlinkMacSystemFont, helvetica, arial, sans-serif'] text-[13px] focus-within:border-[#111]"
)]
pub struct ContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center justify-between leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_10px] text-[#ccc] rounded-[3px]"
)]
pub struct LabelClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center justify-between leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_10px] text-[#111] rounded-[3px] outline-none data-highlighted:bg-[#111] data-highlighted:text-[#fff] data-disabled:text-[#ccc]"
)]
pub struct ItemClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "flex items-center justify-between leading-none cursor-default select-none whitespace-nowrap h-[25px] p-[0px_10px] text-[#111] rounded-[3px] outline-none data-highlighted:bg-[#111] data-highlighted:text-[#fff] data-disabled:text-[#ccc] [&:not([data-highlighted])[data-state=\"open\"]]:bg-[#ccc] [&:not([data-highlighted])[data-state=\"open\"]]:text-[#111]"
)]
pub struct SubTriggerClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "h-[1px] m-[5px_10px] bg-[#ccc]")]
pub struct SeparatorClass {}
