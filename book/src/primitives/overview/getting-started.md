# Getting Started

A quick tutorial to get you up and running with Rust Radix Primitives.

## Implementing a Popover

In this quick tutorial, we will install and style the [Popover](../components/popover.md) component.

### 1. Install the primtive

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-popover
```

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-popover
```

{{#endtab }}
{{#endtabs }}

### 2. Import the parts

Import and structure the parts.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_popover::*;

#[component]
fn PopoverDemo() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>More info</PopoverTrigger>
            <PopoverPortal>
                <PopoverContent>
                    Some more info...
                    <PopoverArrow />
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_popover::*;
use yew::prelude::*;

#[function_component]
fn PopoverDemo() -> Html {
    html! {
        <Popover>
            <PopoverTrigger class="PopoverTrigger">{"More info"}</PopoverTrigger>
            <PopoverPortal>
                <PopoverContent class="PopoverContent">
                    {"Some more info..."}
                    <PopoverArrow />
                </PopoverContent class="PopoverArrow">
            </PopoverPortal>
        </Popover>
    }
}
```

{{#endtab }}
{{#endtabs }}

### 3. Add your styles

Add styles where desired.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
// TODO
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_popover::*;
use yew::prelude::*;

#[function_component]
fn PopoverDemo() -> Html {
    html! {
        <Popover>
            <PopoverTrigger class="PopoverTrigger">{"More info"}</PopoverTrigger>
            <PopoverPortal>
                <PopoverContent class="PopoverContent">
                    {"Some more info..."}
                    <PopoverArrow class="PopoverArrow" />
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}
```

{{#endtab }}
{{#endtabs }}

```css
.PopoverTrigger {
    background-color: white;
    border-radius: 4px;
}

.PopoverContent {
    border-radius: 4px;
    padding: 20px;
    width: 260px;
    background-color: white;
}

.PopoverArrow {
    fill: white;
}
```

### Demo

Here's a complete demo.

TODO

## Summary

The steps above outline briefly what's involved in using a Rust Radix Primitive in your application.

These components are low-level enough to give you control over how you want to wrap them. You're free to introduce your own high-level API to better suit the needs of your team and product.

In a few simple steps, we've implemented a fully accessible Popover component, without having to worry about many of its complexities.

-   Adheres to [WAI-ARIA](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/) design pattern.
-   Can be controlled or uncontrolled.
-   Customize side, alignment, offsets, collision handling.
-   Optionally render a pointing arrow.
-   Focus is fully managed and customizable.
-   Dismissing and layering behavior is highly customizable.

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/overview/getting-started)
