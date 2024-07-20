# Avatar

An image element with a fallback for representing the user.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book"
features = ["avatar"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Automatic and manual control over when the image renders.
-   Fallback part accepts any children.
-   Optionally delay fallback rendering to avoid content flashing.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-avatar
```

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_avatar::*;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage />
            <AvatarFallback />
        </Avatar>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Contains all the parts of an avatar.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default |
| ---------- | ----------------- | ------- |
| `as_child` | `MaybeProp<bool>` | `false` |

{{#endtab }}
{{#endtabs }}

### Image

The image to render. By default it will only render when it has loaded. You can use the `on_loading_status_change` handler if you need more control.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                       | Type                                   | Default |
| -------------------------- | -------------------------------------- | ------- |
| `as_child`                 | `MaybeProp<bool>`                      | `false` |
| `on_loading_status_change` | `Option<Callback<ImageLoadingStatus>>` | -       |

{{#endtab }}
{{#endtabs }}

### Fallback

An element that renders when the image hasn't loaded. This means whilst it's loading, or if there was an error. If you notice a flash during loading, you can provide a `delay_ms` prop to delay its rendering so it only renders for those with slower connections. For more control, use the `on_loading_status_change` handler on `AvatarImage`.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default |
| ---------- | ----------------- | ------- |
| `as_child` | `MaybeProp<bool>` | `false` |
| `delay_ms` | `MaybeProp<i32>`  | -       |

{{#endtab }}
{{#endtabs }}

## Examples

TODO

## See also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/avatar)
