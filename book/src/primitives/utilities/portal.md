# Portal

Renders a subtree in a different part of the DOM.

## Features

-   Render any subtree outside of your App.
-   Appends to `document.body` by default but can be customized to use a different container.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-portal
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-portal)
-   [View on docs.rs](https://docs.rs/radix-leptos-portal/latest/radix_leptos_portal/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/portal)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-portal
```

-   [View on crates.io](https://crates.io/crates/radix-yew-portal)
-   [View on docs.rs](https://docs.rs/radix-yew-portal/latest/radix_yew_portal/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/portal)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_portal::Portal;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Portal />
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_portal::Portal;
use yew::prelude::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
        <Portal />
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Anything you put inside this component will be rendered in a separate `<div>` element. By default, this element will be appended to `document.body` but you can choose a different container by using the `container` or `container_ref` prop.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop            | Type                                       | Default |
| --------------- | ------------------------------------------ | ------- |
| `as_child`      | `MaybeProp<bool>`                          | `false` |
| `container`     | `MaybeProp<SendWrapper<web_sys::Element>>` | -       |
| `container_ref` | `NodeRef<AnyElement>`                      | -       |

{{#endtab }}
{{#tab name="Yew" }}

| Prop            | Type                                       | Default |
| --------------- | ------------------------------------------ | ------- |
| `as_child`      | `Option<Callback<PortalChildProps, Html>>` | -       |
| `container`     | `Option<web_sys::Element>`                 | -       |
| `container_ref` | `Option<NodeRef>`                          | -       |

{{#endtab }}
{{#endtabs }}

## Example

Use the portal primitive.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_portal::Portal;

#[component]
fn Example() -> impl IntoView {
    view! {
        <Portal>Content</Portal>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_portal::Portal;
use yew::prelude::*;

#[function_component]
fn Example() -> Html {
    html! {
        <Portal>{"Content"}</Portal>
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/utilities/portal)
