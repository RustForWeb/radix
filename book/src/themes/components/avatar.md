# Avatar

Profile picture, user initials or fallback icon.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["avatar"]
files = ["src/avatar/avatar.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component inherits props from the [Avatar primitive](../../primitives/components/avatar.md).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                                 | Default               |
| --------------- | ------------------------------------ | --------------------- |
| `as_child`      | `Option<Callback<AvatarChildProps>>` | -                     |
| `size`          | `Responsive<1..9>`                   | `3`                   |
| `variant`       | `AvatarVariant`                      | `AvatarVariant::Soft` |
| `color`         | `Option<AccentColor>`                | -                     |
| `high_contrast` | `Option<bool>`                       | -                     |
| `radius`        | `Option<Radius>`                     | -                     |
| `fallback`      | `Html`                               | -                     |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size of the avatar.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["avatar"]
files = ["src/avatar/avatar_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style of the avatar.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["avatar"]
files = ["src/avatar/avatar_variant.rs"]
show_files = true
url_fragment = "#/variant"
```

{{#endtab }}
{{#endtabs }}

### Color

Use the `color` prop to assign a specific [color](../theme/color.md).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["avatar"]
files = ["src/avatar/avatar_color.rs"]
show_files = true
url_fragment = "#/color"
```

{{#endtab }}
{{#endtabs }}

### High-Contrast

Use the `high_contrast` prop to increase color contrast with the background.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["avatar"]
files = ["src/avatar/avatar_high_contrast.rs"]
show_files = true
url_fragment = "#/high-contrast"
```

{{#endtab }}
{{#endtabs }}

### Radius

Use the `radius` prop to assign a specific radius value.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["avatar"]
files = ["src/avatar/avatar_radius.rs"]
show_files = true
url_fragment = "#/radius"
```

{{#endtab }}
{{#endtabs }}

### Fallback

Use the `fallback` prop to modify the rendered fallback element.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["avatar"]
files = ["src/avatar/avatar_fallback.rs"]
show_files = true
url_fragment = "#/fallback"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/avatar)
