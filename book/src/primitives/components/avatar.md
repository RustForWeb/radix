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

TODO

### Image

TODO

### Fallback

TODO

## Examples

TODO
