# Label

Renders an accessible label associated with controls.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book"
features = ["label"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Text selection is prevented when double clicking label.
-   Supports nested controls.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-label
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-label)
-   [View on docs.rs](https://docs.rs/radix-leptos-label/latest/radix_leptos_label/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/label)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_label::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Label />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains the content for the label.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop            | Type                           | Default |
| --------------- | ------------------------------ | ------- |
| `as_child`      | `MaybeProp<bool>`              | `false` |
| `on_mouse_down` | `Option<Callback<MouseEvent>>` | -       |

{{#endtab }}
{{#endtabs }}

## Accessibility

This component is based on the native `label` element, it will automatically apply the correct labelling when wrapping controls or using the `for` attribute. For your own custom controls to work correctly, ensure they use native elements such as `button` or `input` as a base.

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/label)
