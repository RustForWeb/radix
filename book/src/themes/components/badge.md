# Badge

Stylized badge element.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["badge"]
files = ["src/badge/badge.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `span` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                                | Default               |
| --------------- | ----------------------------------- | --------------------- |
| `as_child`      | `Option<Callback<BadgeChildProps>>` | -                     |
| `size`          | `Responsive<1..3>`                  | `1`                   |
| `variant`       | `BadgeVariant`                      | `BadgeVariant::Solid` |
| `color`         | `Option<AccentColor>`               | -                     |
| `high_contrast` | `Option<bool>`                      | -                     |
| `radius`        | `Option<Radius>`                    | -                     |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["badge"]
files = ["src/badge/badge_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["badge"]
files = ["src/badge/badge_variant.rs"]
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
features = ["badge"]
files = ["src/badge/badge_color.rs"]
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
features = ["badge"]
files = ["src/badge/badge_high_contrast.rs"]
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
features = ["badge"]
files = ["src/badge/badge_radius.rs"]
show_files = true
url_fragment = "#/radius"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/badge)
