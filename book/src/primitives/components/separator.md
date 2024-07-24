# Separator

Visually or semantically separates content.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book"
features = ["separator"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Supports horizontal and vertical orientations.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-separator
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-separator)
-   [View on docs.rs](https://docs.rs/radix-leptos-separator/latest/radix_leptos_separator/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/separator)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_separator::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Separator />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

The separator.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop          | Type                     | Default                   |
| ------------- | ------------------------ | ------------------------- |
| `as_child`    | `MaybeProp<bool>`        | `false`                   |
| `orientation` | `MaybeProp<Orientation>` | `Orientation::Horizontal` |
| `decorative`  | `MaybeProp<bool>`        | `false`                   |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute       | Values                       |
| -------------------- | ---------------------------- |
| `[data-orientation]` | `"horizontal" \| "vertical"` |

# Accessibility

Adheres to the [`separator` role requirements](https://www.w3.org/TR/wai-aria-1.2/#separator).

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/separator)
