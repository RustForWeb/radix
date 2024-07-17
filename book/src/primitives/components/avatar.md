# Avatar

An image element with a fallback for representing the user.

```toml,trunk
package = "radix-leptos-book"
features = ["avatar"]
```

## Features

-   Automatic and manual control over when the image renders.
-   Fallback part accepts any children.
-   Optionally delay fallback rendering to avoid content flashing.

## Installation

Install the component from your command line.

```shell
cargo add radix-leptos-avatar
```

## Anatomy

Import all parts and piece them together.

```rust,ignore
use radix_leptos_avatar::*;

{
    view! {
        <Avatar>
            <AvatarImage />
            <AvatarFallback />
        </Avatar>
    }
}
```

## API Reference

### Root

TODO

### Image

TODO

### Fallback

TODO

## Examples

TODO
