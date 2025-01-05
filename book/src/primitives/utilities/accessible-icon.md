# Accessible Icon

Makes icons accessible by adding a label.

## Features

-   Quickly make any icon accessible by wrapping it and providing a meaningful label.
-   No visual difference, but announced correctly by screen readers.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-accessible-icon
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-accessible-icon)
-   [View on docs.rs](https://docs.rs/radix-leptos-accessible-icon/latest/radix_leptos_accessible_icon/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/accessible-icon)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_accessible_icon::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <AccessibleIcon />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains the icon to make accessible.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop    | Type             | Default |
| ------- | ---------------- | ------- |
| `label` | `Signal<String>` | -       |

{{#endtab }}
{{#endtabs }}

## Accessibility

Most icons or icon systems come with no accessibility built-in. For example, the same visual **cross** icon may in fact mean **"close"** or **"delete"**.
This component lets you give meaning to icons used throughout your app.

This is built with [Visually Hidden](./visually-hidden.md).

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/utilities/accessible-icon)
