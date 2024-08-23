# Aspect Ratio

Displays content within a desired ratio.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book"
features = ["aspect-ratio"]
files = ["src/aspect_ratio.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Accepts any custom ratio.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-aspect-ratio
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-aspect-ratio)
-   [View on docs.rs](https://docs.rs/radix-leptos-aspect-ratio/latest/radix_leptos_aspect_ratio/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/aspect-ratio)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_aspect_ratio::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <AspectRatio />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains the content you want to constrain to a given ratio.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default |
| ---------- | ----------------- | ------- |
| `as_child` | `MaybeProp<bool>` | `false` |
| `ratio`    | `MaybeProp<f64>`  | `1.0`   |

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/aspect-ratio)
