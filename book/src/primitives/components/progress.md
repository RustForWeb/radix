# Progress

Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book"
features = ["progress"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Provides context for assistive technology to read the progress of a task.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-progress
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-progress)
-   [View on docs.rs](https://docs.rs/radix-leptos-progress/latest/radix_leptos_progress/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/progress)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_progress::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Progress>
            <ProgressIndicator />
        </Progress>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains all of the progress parts.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop              | Type                                   | Default |
| ----------------- | -------------------------------------- | ------- |
| `as_child`        | `MaybeProp<bool>`                      | `false` |
| `value`           | `MaybeProp<f64>`                       | -       |
| `max`             | `MaybeProp<f64>`                       | `100.0` |
| `get_value_label` | `Option<Callback<(f64, f64), String>>` | -       |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute | Values                                       |
| -------------- | -------------------------------------------- |
| `[data-state]` | `"complete" \| "indeterminate" \| "loading"` |
| `[data-value]` | The current value                            |
| `[data-max]`   | The max value                                |

### Indicator

Used to show the progress visually. It also makes progress accessible to assistive technologies.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default |
| ---------- | ----------------- | ------- |
| `as_child` | `MaybeProp<bool>` | `false` |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute | Values                                       |
| -------------- | -------------------------------------------- |
| `[data-state]` | `"complete" \| "indeterminate" \| "loading"` |
| `[data-value]` | The current value                            |
| `[data-max]`   | The max value                                |

## Accessibility

Adheres to the [`progressbar` role requirements](https://www.w3.org/WAI/ARIA/apg/patterns/meter/).

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/progress)
