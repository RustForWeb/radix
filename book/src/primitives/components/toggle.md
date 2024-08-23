# Toggle

A two-state button that can be either on or off.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book"
features = ["toggle"]
files = ["src/toggle.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Full keyboard navigation.
-   Can be controlled or uncontrolled.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-toggle
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-toggle)
-   [View on docs.rs](https://docs.rs/radix-leptos-toggle/latest/radix_leptos_toggle/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/toggle)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_toggle::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Toggle />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

The toggle.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                | Type                     | Default |
| ------------------- | ------------------------ | ------- |
| `as_child`          | `MaybeProp<bool>`        | `false` |
| `default_pressed`   | `MaybeProp<bool>`        | -       |
| `pressed`           | `MaybeProp<bool>`        | -       |
| `on_pressed_change` | `Option<Callback<bool>>` | -       |
| `disabled`          | `MaybeProp<bool>`        | -       |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute    | Values                |
| ----------------- | --------------------- |
| `[data-state]`    | `"on" \| "off"`       |
| `[data-disabled]` | Present when disabled |

## Accessibility

### Keyboard Interactions

| Key     | Description                       |
| ------- | --------------------------------- |
| `Space` | Activates/deactivates the toggle. |
| `Enter` | Activates/deactivates the toggle. |

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/toggle)
