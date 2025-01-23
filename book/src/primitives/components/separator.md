# Separator

Visually or semantically separates content.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book-primitives"
features = ["separator"]
files = ["src/separator.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["separator"]
files = ["src/separator.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

- Supports horizontal and vertical orientations.

## Installation

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-separator
```

- [View on crates.io](https://crates.io/crates/radix-leptos-separator)
- [View on docs.rs](https://docs.rs/radix-leptos-separator/latest/radix_leptos_separator/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/separator)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-separator
```

- [View on crates.io](https://crates.io/crates/radix-yew-separator)
- [View on docs.rs](https://docs.rs/radix-yew-separator/latest/radix_yew_separator/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/separator)

{{#endtab }}
{{#endtabs }}

## Anatomy

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
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;
use radix_yew_separator::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
        <Separator />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

The separator component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop          | Type                     | Default                   | Description                                                                                          |
|---------------|--------------------------|---------------------------|------------------------------------------------------------------------------------------------------|
| `as_child`    | `MaybeProp<bool>`        | `false`                   | If `true`, renders only its children without the default wrapper.                                    |
| `orientation` | `MaybeProp<Orientation>` | `Orientation::Horizontal` | Determines whether the separator is horizontal or vertical.                                          |
| `decorative`  | `MaybeProp<bool>`        | `false`                   | If `true`, the separator is considered purely visual, and does not appear in the accessibility tree. |

{{#endtab }}
{{#tab name="Yew" }}

| Prop          | Type                                          | Default                   | Description                                                                                          |
|---------------|-----------------------------------------------|---------------------------|------------------------------------------------------------------------------------------------------|
| `as_child`    | `Option<Callback<SeparatorChildProps, Html>>` | -                         | If present, renders only its children without the default wrapper.                                   |
| `orientation` | `Orientation`                                 | `Orientation::Horizontal` | Determines whether the separator is horizontal or vertical.                                          |
| `decorative`  | `bool`                                        | `false`                   | If `true`, the separator is considered purely visual, and does not appear in the accessibility tree. |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

**Data attributes**:

| Data Attribute       | Values                         |
|----------------------|--------------------------------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

## Accessibility

Adheres to the [separator role requirements](https://www.w3.org/TR/wai-aria-1.2/#separator).

## See Also

- [Radix documentation](https://www.radix-ui.com/primitives/docs/components/separator)
