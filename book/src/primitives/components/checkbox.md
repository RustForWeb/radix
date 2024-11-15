# Checkbox

A control that allows the user to toggle between checked and not checked.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book-primitives"
features = ["checkbox"]
files = ["src/checkbox.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["checkbox"]
files = ["src/checkbox.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Supports indeterminate state.
-   Full keyboard navigation.
-   Can be controlled or uncontrolled.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-checkbox
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-checkbox)
-   [View on docs.rs](https://docs.rs/radix-leptos-checkbox/latest/radix_leptos_checkbox/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/checkbox)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-checkbox
```

-   [View on crates.io](https://crates.io/crates/radix-yew-checkbox)
-   [View on docs.rs](https://docs.rs/radix-yew-checkbox/latest/radix_yew_checkbox/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/checkbox)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_checkbox::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Checkbox>
            <CheckboxIndicator />
        </Checkbox>
    }
}
```

{{#endtab }}
{{#tab name="Leptos" }}

```rust,ignore
use radix_yew_checkbox::*;
use yew::prelude::*;

#[function_component]
fn Anatomy() -> Htm; {
    html! {
        <Checkbox>
            <CheckboxIndicator />
        </Checkbox>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains all the parts of a checkbox. An `input` will also render when used within a `form` to ensure events propagate correctly.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                | Type                      | Default |
| ------------------- | ------------------------- | ------- |
| `as_child`          | `MaybeProp<bool>`         | `false` |
| `default_checked`   | `MaybeProp<CheckedState>` | -       |
| `checked`           | `MaybeProp<CheckedState>` | -       |
| `on_checked_change` | `Option<Callback<bool>>`  | -       |
| `disabled`          | `MaybeProp<bool>`         | -       |
| `required`          | `MaybeProp<bool>`         | -       |
| `name`              | `MaybeProp<String>`       | -       |
| `value`             | `MaybeProp<String>`       | `"on"`  |

{{#endtab }}
{{#tab name="Yew" }}

| Prop                | Type                                         | Default |
| ------------------- | -------------------------------------------- | ------- |
| `as_child`          | `Option<Callback<CheckboxChildProps, Html>>` | -       |
| `default_checked`   | `Option<CheckedState>`                       | -       |
| `checked`           | `Option<CheckedState>`                       | -       |
| `on_checked_change` | `Callback<bool>`                             | -       |
| `disabled`          | `Option<bool>`                               | -       |
| `required`          | `Option<bool>`                               | -       |
| `name`              | `Option<String>`                             | -       |
| `value`             | `String`                                     | `"on"`  |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute    | Values                                        |
| ----------------- | --------------------------------------------- |
| `[data-state]`    | `"checked" \| "unchecked" \| "indeterminate"` |
| `[data-disabled]` | Present when disabled                         |

### Indicator

Renders when the checkbox is in a checked or indeterminate state. You can style this element directly, or you can use it as a wrapper to put an icon into, or both.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop          | Type              | Default |
| ------------- | ----------------- | ------- |
| `as_child`    | `MaybeProp<bool>` | `false` |
| `force_mount` | `MaybeProp<bool>` | -       |

{{#endtab }}
{{#tab name="Yew" }}

| Prop          | Type                                              | Default |
| ------------- | ------------------------------------------------- | ------- |
| `as_child`    | `Option<Callback<CheckboxThumbChildProps, Html>>` | -       |
| `force_mount` | `Option<bool>`                                    | -       |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute    | Values                                        |
| ----------------- | --------------------------------------------- |
| `[data-state]`    | `"checked" \| "unchecked" \| "indeterminate"` |
| `[data-disabled]` | Present when disabled                         |

## Examples

### Indeterminate

You can set the checkbox to `CheckedState::Indeterminate` by taking control of its state.

TODO

## Accessibility

Adheres to the [tri-state Checkbox WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/switch/).

### Keyboard Interactions

| Key     | Description                   |
| ------- | ----------------------------- |
| `Space` | Checks/unchecks the checkbox. |

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/checkbox)
