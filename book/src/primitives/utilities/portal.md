# Portal

Renders a subtree in a different part of the DOM.

## Features

-   Render any React subtree outside of your App.
-   Appends to `document.body` by default but can be customized to use a different container.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
# CSR
cargo add radix-leptos-portal --features csr

# Hydrate
cargo add radix-leptos-portal --features hydrate

# SSR
cargo add radix-leptos-portal --features ssr
```

-   [View on crates.io](https://crates.io/crates/radix-leptos-portal)
-   [View on docs.rs](https://docs.rs/radix-leptos-portal/latest/radix_leptos_portal/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/portal)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import the component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_portal::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
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

| Prop            | Type                          | Default |
| --------------- | ----------------------------- | ------- |
| `as_child`      | `MaybeProp<bool>`             | `false` |
| `container`     | `MaybeProp<web_sys::Element>` | -       |
| `container_ref` | `NodeRef<AnyElement>`         | -       |

{{#endtab }}
{{#endtabs }}

## Example

Use the portal primitive.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_portal::*;

#[component]
fn Example() -> impl IntoView {
    view! {
        <Portal>Content</Portal>
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/utilities/portal)
