# Icons

A crisp set of 15×15 icons. All icons are available as individual components.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book-icons"
features = ["icons"]
files = ["src/icons.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-icons"
features = ["icons"]
files = ["src/icons.rs"]
```

{{#endtab }}
{{#endtabs }}

## Installation

Install the icons from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-icons
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-icons)
-   [View on docs.rs](https://docs.rs/radix-leptos-icons/latest/radix_leptos_icons/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/icons/leptos)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-icons
```

-   [View on crates.io](https://crates.io/crates/radix-yew-icons)
-   [View on docs.rs](https://docs.rs/radix-yew-icons/latest/radix_yew_icons/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/icons/yew)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the icons.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_icons::{FaceIcon, ImageIcon, SunIcon};

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <FaceIcon />
        <SunIcon />
        <ImageIcon />
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;
use radix_yew_icons::{FaceIcon, ImageIcon, SunIcon};

#[function_component]
fn Anatomy() -> Html {
    html! {
        <>
            <FaceIcon />
            <SunIcon />
            <ImageIcon />
        </>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop    | Type                | Default          |
| ------- | ------------------- | ---------------- |
| `color` | `MaybeProp<String>` | `"currentColor"` |

{{#endtab }}
{{#tab name="Yew" }}

| Prop    | Type        | Default          |
| ------- | ----------- | ---------------- |
| `color` | `AttrValue` | `"currentColor"` |

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/icons)
