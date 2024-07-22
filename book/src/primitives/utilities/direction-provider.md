# Direction Provider

Wraps your app to provide global reading direction.

## Features

-   Enables all primitives to inherit global reading direction.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-direction-provider
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-direction-provider)
-   [View on docs.rs](https://docs.rs/radix-leptos-direction-provider/latest/radix_leptos_direction_provider/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/direction)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_direction_provider::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <DirectionProvider />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

When creating localized apps that require right-to-left (RTL) reading direction, you need to wrap your application with the `DirectionProvider` component to ensure all of the primitives adjust their behavior based on the `dir` prop.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop  | Type                     | Default |
| ----- | ------------------------ | ------- |
| `dir` | `MaybeSignal<Direction>` | -       |

{{#endtab }}
{{#endtabs }}

## Example

Use the direction provider.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_direction_provider::*;

#[component]
fn Example() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl>
            /* your app */
        </DirectionProvider>
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/utilities/direction-provider)
