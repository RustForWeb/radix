# Aspect Ratio

Displays content within a desired ratio.

{{#tabs global="framework"}}
{{#tab name="Leptos"}}

```toml,trunk
package = "radix-leptos-book-primitives"
features = ["aspect-ratio"]
files = ["src/aspect_ratio.rs"]
```

{{#endtab}}
{{#tab name="Yew"}}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["aspect-ratio"]
files = ["src/aspect_ratio.rs"]
```

{{#endtab}}
{{#endtabs}}

## Features

- Accepts any custom ratio.

## Installation

Install the component from your command line.

{{#tabs global="framework"}}
{{#tab name="Leptos"}}

```shell
cargo add radix-leptos-aspect-ratio
```

- [View on crates.io](https://crates.io/crates/radix-leptos-aspect-ratio)
- [View on docs.rs](https://docs.rs/radix-leptos-aspect-ratio/latest/radix_leptos_aspect_ratio/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/aspect-ratio)

{{#endtab}}
{{#tab name="Yew"}}

```shell
cargo add radix-yew-aspect-ratio
```

- [View on crates.io](https://crates.io/crates/radix-yew-aspect-ratio)
- [View on docs.rs](https://docs.rs/radix-yew-aspect-ratio/latest/radix_yew_aspect_ratio/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/aspect-ratio)

{{#endtab}}
{{#endtabs}}

## Anatomy

{{#tabs global="framework"}}
{{#tab name="Leptos"}}

```rust,ignore
use leptos::*;
use radix_leptos_aspect_ratio as AspectRatio;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <AspectRatio::Root ratio=16.0/9.0>
            <div>"Constrained within the 16:9 ratio!"</div>
        </AspectRatio::Root>
    }
}
```

{{#endtab}}
{{#tab name="Yew"}}

```rust,ignore
use radix_yew_aspect_ratio::*;
use yew::prelude::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
        <AspectRatio>
            {"Constrained within the desired ratio!"}
        </AspectRatio>
    }
}
```

{{#endtab}}
{{#endtabs}}

## API Reference

### Root

Contains the content you want to constrain to a given ratio.

{{#tabs global="framework"}}
{{#tab name="Leptos"}}

| Prop       | Type              | Default |
|------------|-------------------|---------|
| `as_child` | `MaybeProp<bool>` | `false` |
| `ratio`    | `MaybeProp<f64>`  | `1.0`   |
| `node_ref` | `AnyNodeRef`      | -       |

### Usage

To use the `Root` alias for the `AspectRatio` component, import it and use `AspectRatio::Root` in your view.

```rust,ignore
use leptos::*;
use radix_leptos_aspect_ratio as AspectRatio;

#[component]
fn Example() -> impl IntoView {
    view! {
        <AspectRatio::Root ratio=4.0/3.0>
            <img src="path/to/image.jpg" alt="Example Image" />
        </AspectRatio::Root>
    }
}
```

{{#endtab}}
{{#tab name="Yew"}}

<!-- TODO: Add or update Yew-specific props if needed -->

| Prop    | Type  | Default |
|---------|-------|---------|
| `ratio` | `f64` | `1.0`   |

{{#endtab}}
{{#endtabs}}

## See Also

- [Radix documentation](https://www.radix-ui.com/primitives/docs/components/aspect-ratio)

```