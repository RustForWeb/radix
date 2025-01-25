# Avatar

An image element with a fallback for representing the user.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "radix-leptos-book-primitives"
features = ["avatar"]
files = ["src/avatar.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["avatar"]
files = ["src/avatar.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

- Automatic and manual control over when the image renders.
- Fallback part accepts any children.
- Optionally delay fallback rendering to avoid content flashing.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
cargo add radix-leptos-avatar
```

- [View on crates.io](https://crates.io/crates/radix-leptos-avatar)
- [View on docs.rs](https://docs.rs/radix-leptos-avatar/latest/radix_leptos_avatar/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/leptos/avatar)

{{#endtab }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-avatar
```

- [View on crates.io](https://crates.io/crates/radix-yew-avatar)
- [View on docs.rs](https://docs.rs/radix-yew-avatar/latest/radix_yew_avatar/)
- [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/avatar)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_avatar::primitive as Avatar;

#[component]
fn Anatomy() -> impl IntoView {
    view! {
        <Avatar::Root>
            <Avatar::Image src="https://example.com/avatar.png" />
            <Avatar::Fallback delay_ms=500>
                {"AB"}
            </Avatar::Fallback>
        </Avatar::Root>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_avatar::*;
use yew::prelude::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
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

| Prop       | Type              | Default | Description                                                                              |
|------------|-------------------|---------|------------------------------------------------------------------------------------------|
| `as_child` | `MaybeProp<bool>` | `false` | If `true`, renders only its children without a `<span>` wrapper.                         |
| `node_ref` | `AnyNodeRef`      | -       | Optional reference to the underlying `<span>` element.                                   |
| `children` | `TypedChildrenFn` | -       | The content of the `Avatar` component (commonly `Avatar::Image` and `Avatar::Fallback`). |

{{#endtab }}
{{#tab name="Yew" }}

| Prop       | Type                                       | Default |
|------------|--------------------------------------------|---------|
| `as_child` | `Option<Callback<AvatarChildProps, Html>>` | -       |

{{#endtab }}
{{#endtabs }}

### AvatarImage

Displays the image. By default, it only renders if the image successfully loads.  
Use the `on_loading_status_change` callback or check the context (see `use_avatar_context`) if you need more control.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop                       | Type                                | Default | Description                                                                                                                                     |
|----------------------------|-------------------------------------|---------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| `src`                      | `MaybeProp<String>`                 | -       | The source URL for the avatar image.                                                                                                            |
| `referrer_policy`          | `MaybeProp<String>`                 | -       | Sets the `referrerpolicy` attribute of the `<img>` tag if needed.                                                                               |
| `as_child`                 | `MaybeProp<bool>`                   | `false` | If `true`, renders only its children without an `<img>` tag.                                                                                    |
| `on_loading_status_change` | `MaybeCallback<ImageLoadingStatus>` | -       | Callback to be fired when the image loading status changes. Receives an `ImageLoadingStatus` enum value (`Idle`, `Loading`, `Loaded`, `Error`). |
| `node_ref`                 | `AnyNodeRef`                        | -       | Optional reference to the underlying `<img>` element.                                                                                           |
| `children`                 | `Option<ChildrenFn>`                | -       | If `as_child = true`, can pass child elements to be used instead of an `<img>`.                                                                 |

{{#endtab }}
{{#tab name="Yew" }}

| Prop                       | Type                                            | Default |
|----------------------------|-------------------------------------------------|---------|
| `as_child`                 | `Option<Callback<AvatarImageChildProps, Html>>` | -       |
| `on_loading_status_change` | `Callback<ImageLoadingStatus>`                  | -       |

{{#endtab }}
{{#endtabs }}

### AvatarFallback

Renders its children while the image is loading or if it fails to load.  
Use `delay_ms` to avoid flashing the fallback for users with a fast connection.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

| Prop       | Type              | Default | Description                                                                                                                                |
|------------|-------------------|---------|--------------------------------------------------------------------------------------------------------------------------------------------|
| `delay_ms` | `MaybeProp<i32>`  | -       | Delay (in milliseconds) before showing the fallback content. If none is provided, fallback shows up immediately if the image isn't loaded. |
| `as_child` | `MaybeProp<bool>` | `false` | If `true`, renders only its children without a `<span>` wrapper.                                                                           |
| `node_ref` | `AnyNodeRef`      | -       | Optional reference to the `<span>` element.                                                                                                |
| `children` | `TypedChildrenFn` | -       | Typically, initials or an icon to display in place of the image.                                                                           |

{{#endtab }}
{{#tab name="Yew" }}

| Prop       | Type                                               | Default |
|------------|----------------------------------------------------|---------|
| `as_child` | `Option<Callback<AvatarFallbackChildProps, Html>>` | -       |
| `delay_ms` | `Option<i32>`                                      | -       |

{{#endtab }}
{{#endtabs }}

## Examples

**Basic Usage**

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_avatar::primitive as Avatar;

#[component]
fn DemoAvatar() -> impl IntoView {
    view! {
        <Avatar::Root>
            <Avatar::Image src="https://example.com/my-avatar.png" />
            <Avatar::Fallback delay_ms=300>
                {"JD"}
            </Avatar::Fallback>
        </Avatar::Root>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}
TODO
{{#endtab }}
{{#endtabs }}

**Custom Callback**

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_avatar::primitive as Avatar;
use radix_leptos_avatar::ImageLoadingStatus;

#[component]
fn AvatarWithCallback() -> impl IntoView {
    let on_status_change = move |status: ImageLoadingStatus| {
        match status {
            ImageLoadingStatus::Idle => log!("Avatar Image: Idle"),
            ImageLoadingStatus::Loading => log!("Avatar Image: Loading"),
            ImageLoadingStatus::Loaded => log!("Avatar Image: Loaded"),
            ImageLoadingStatus::Error => log!("Avatar Image: Failed to load"),
        }
    };

    view! {
        <Avatar::Root>
            <Avatar::Image
                src="https://example.com/my-avatar.png"
                on_loading_status_change=on_status_change
            />
            <Avatar::Fallback>
                {"Fallback"}
            </Avatar::Fallback>
        </Avatar::Root>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}
TODO
{{#endtab }}
{{#endtabs }}

## See Also

- [Radix UI documentation](https://www.radix-ui.com/primitives/docs/components/avatar)
- [Repository and more examples](https://github.com/RustForWeb/radix)  