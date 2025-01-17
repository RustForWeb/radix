# Accessible Icon

Makes icons accessible by adding a label.

## Features

- Quickly make any icon accessible by wrapping it and providing a meaningful label.
- No visual difference, but announced correctly by screen readers.

## Installation

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-accessible-icon
```

- [View on crates.io](https://crates.io/crates/radix-leptos-accessible-icon)
- [View on docs.rs](https://docs.rs/radix-leptos-accessible-icon/latest/radix_leptos_accessible_icon/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/accessible-icon)

{{#endtab }}
{{#tab name="Yew" }}

```shell
# TODO: Implement an official Yew crate for accessible icons if missing
# cargo add radix-yew-accessible-icon (when available)
```

- TODO: Provide crates.io, docs.rs, and source links when available.

{{#endtab }}
{{#endtabs }}

## Anatomy

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_accessible_icon::primitive as AccessibleIcon;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <AccessibleIcon::Root label="Close">
            // Your icon element here (e.g., <svg>)
            // aria-hidden="true", focusable="false" are applied automatically
            <svg viewBox="0 0 24 24">
                // path definitions...
            </svg>
        </AccessibleIcon::Root>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
// TODO: Provide a Yew example when the crate is available or supported
use yew::prelude::*;

// Possibly something like this in the future:
// use radix_yew_accessible_icon::primitive as AccessibleIcon;
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains the icon you want to make accessible.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type                                     | Default | Description                                                                                                               |
|------------|------------------------------------------|---------|---------------------------------------------------------------------------------------------------------------------------|
| `label`    | `MaybeProp<String>`                      | –       | The accessible label for the icon. It will be visually hidden but announced to screen readers.                            |
| `children` | `TypedChildren<impl IntoView + 'static>` | –       | Your icon (SVG or similar). Extra attributes like `aria-hidden="true"` and `focusable="false"` are automatically applied. |

{{#endtab }}
{{#tab name="Yew" }}

| Prop  | Type | Default | Description           |
|-------|------|---------|-----------------------|
| label | TODO | -       | TODO for Yew support. |
| child | TODO | -       | TODO for Yew support. |

{{#endtab }}
{{#endtabs }}

## Accessibility

Most icons or icon systems come with no accessibility built-in. For example, the same visual **cross** icon might mean *
*"close"** or **"delete"**. By wrapping your icon in `<AccessibleIcon::Root>` (Leptos/Yew), you provide a meaningful
label that screen readers will announce, while the icon itself is hidden from assistive technology via
`aria-hidden="true"`.

This is built with [Visually Hidden](../utilities/visually-hidden) so labels are hidden visually but still accessible to
screen readers.

## Example (Leptos)

```rust,ignore
use leptos::prelude::*;
use radix_leptos_visually_hidden::primitive as VisuallyHidden;
use radix_leptos_accessible_icon::primitive as AccessibleIcon;

#[component]
fn CloseIcon() -> impl IntoView {
    view! {
        <AccessibleIcon::Root label="Close">
            <svg width="24" height="24" viewBox="0 0 24 24">
                <path d="M6 6 L18 18 M6 18 L18 6" stroke="currentColor" stroke-width="2" />
            </svg>
        </AccessibleIcon::Root>
    }
}
```

> **Yew**: TODO – Provide a working example if/when the Yew version is complete.

## See Also

- [Radix UI Accessible Icon documentation](https://www.radix-ui.com/primitives/docs/utilities/accessible-icon)
- [Repository and more examples](https://github.com/RustForWeb/radix)