# Label

Renders an accessible label associated with controls.

## Features

- Text selection is prevented when double-clicking label
- Supports nested controls
- Composable with other elements using `as_child`
- Handles mouse events with proper focus management

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-label
```

- [View on crates.io](https://crates.io/crates/radix-leptos-label)
- [View on docs.rs](https://docs.rs/radix-leptos-label/latest/radix_leptos_label/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/label)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-label
```

- [View on crates.io](https://crates.io/crates/radix-yew-label)
- [View on docs.rs](https://docs.rs/radix-yew-label/latest/radix_yew_label/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/label)

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
        <Label>
            "Label text"
        </Label>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_label::*;
use yew::prelude::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
        <Label>
            {"Label text"}
        </Label>
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

| Prop            | Type                                       | Default | Description                                                       |
|-----------------|--------------------------------------------|---------|-------------------------------------------------------------------|
| `children`      | `TypedChildrenFn<impl IntoView + 'static>` | -       | The content to be rendered inside the label                       |
| `as_child`      | `MaybeProp<bool>`                          | `false` | Change the default rendered element for the one passed as a child |
| `on_mouse_down` | `MaybeCallback<MouseEvent>`                | -       | Event handler for mousedown events                                |
| `node_ref`      | `AnyNodeRef`                               | -       | A reference to the underlying DOM node                            |

{{#endtab }}
{{#tab name="Yew" }}

| Prop            | Type                                      | Default | Description                         |
|-----------------|-------------------------------------------|---------|-------------------------------------|
| `as_child`      | `Option<Callback<LabelChildProps, Html>>` | -       | Change the default rendered element |
| `on_mouse_down` | `Option<Callback<MouseEvent>>`            | -       | Event handler for mousedown events  |

{{#endtab }}
{{#endtabs }}

## Behavior

The Label component includes the following built-in behaviors:

- Prevents text selection when double-clicking
- Ignores mousedown events on nested controls (button, input, select, textarea)
- Supports composition via `as_child` prop
- Can be associated with controls both by wrapping them or using the `for` attribute

## Accessibility

This component is based on the native `label` element, it will automatically apply the correct labelling when wrapping
controls or using the `for` attribute. For your own custom controls to work correctly, ensure they use native elements
such as `button` or `input` as a base.

When using custom components within labels:

- Ensure they ultimately render to native form elements
- Verify that click events properly propagate through to the underlying control
- Test with screen readers to confirm proper label association

## See Also

- [Radix documentation](https://www.radix-ui.com/primitives/docs/components/label)