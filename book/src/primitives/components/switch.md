# Switch

A control that allows the user to toggle between checked and not checked.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book"
features = ["switch"]
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
cargo add radix-leptos-switch
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-switch)
-   [View on docs.rs](https://docs.rs/radix-leptos-switch/latest/radix_leptos_switch/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/switch)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_switch::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Switch>
            <SwitchThumb />
        </Switch>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains all the parts of a switch. An `input` will also render when used within a `form` to ensure events propagate correctly.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                | Type                     | Default |
| ------------------- | ------------------------ | ------- |
| `as_child`          | `MaybeProp<bool>`        | `false` |
| `default_checked`   | `MaybeProp<bool>`        | -       |
| `checked`           | `MaybeProp<bool>`        | -       |
| `on_checked_change` | `Option<Callback<bool>>` | -       |
| `disabled`          | `MaybeProp<bool>`        | -       |
| `required`          | `MaybeProp<bool>`        | -       |
| `name`              | `MaybeProp<String>`      | -       |
| `value`             | `MaybeProp<String>`      | `"on"`  |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute    | Values                     |
| ----------------- | -------------------------- |
| `[data-state]`    | `"checked" \| "unchecked"` |
| `[data-disabled]` | Present when disabled      |

### Thumb

The thumb that is used to visually indicate whether the switch is on or off.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default |
| ---------- | ----------------- | ------- |
| `as_child` | `MaybeProp<bool>` | `false` |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute    | Values                     |
| ----------------- | -------------------------- |
| `[data-state]`    | `"checked" \| "unchecked"` |
| `[data-disabled]` | Present when disabled      |

## Accessibility

Adheres to the [`switch` role requirements](https://www.w3.org/WAI/ARIA/apg/patterns/switch/).

### Keyboard Interactions

| Key     | Description                    |
| ------- | ------------------------------ |
| `Space` | Toggles the component's state. |
| `Enter` | Toggles the component's state. |

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/switch)
