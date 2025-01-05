# Visually Hidden

Hides content from the screen in an accessible way.

## Features

-   Visually hides content while preserving it for assistive technology.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-visually-hidden
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-visually-hidden)
-   [View on docs.rs](https://docs.rs/radix-leptos-visually-hidden/latest/radix_leptos_visually_hidden/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/visually-hidden)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-visually-hidden
```

-   [View on crates.io](https://crates.io/crates/radix-yew-visually-hidden)
-   [View on docs.rs](https://docs.rs/radix-yew-visually-hidden/latest/radix_yew_visually_hidden/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/visually-hidden)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_visually_hidden::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <VisuallyHidden />
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_visually_hidden::*;
use yew::prelude::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
        <VisuallyHidden />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Anything you put inside this component will be hidden from the screen but will be announced by screen readers.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

<i>No props.</i>

{{#endtab }}
{{#tab name="Yew" }}

| Prop       | Type                                               | Default |
| ---------- | -------------------------------------------------- | ------- |
| `as_child` | `Option<Callback<VisuallyHiddenChildProps, Html>>` | -       |

{{#endtab }}
{{#endtabs }}

## Example

Use the visually hidden primitive.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_icons::GearIcon;
use radix_leptos_visually_hidden::*;

#[component]
fn Example() -> impl IntoView {
    view! {
        <button>
            <GearIcon />
            <VisuallyHidden>"Settings"</VisuallyHidden>
        </button>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_icons::GearIcon;
use radix_yew_visually_hidden::*;
use yew::prelude::*;

#[component]
fn Example() -> impl IntoView {
    view! {
        <button>
            <GearIcon />
            <VisuallyHidden>{"Settings"}</VisuallyHidden>
        </button>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Accessibility

This is useful in certain scenarios as an alternative to traditional labelling with `aria-label` or `aria-labelledby`.

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/utilities/visually-hidden)
