# Separator

Visually or semantically separates content.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["separator"]
files = ["src/separator/separator.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component inherits props from the [Separator primitive](../../primitives/components/separator.md) and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop          | Type                   | Default                          |
| ------------- | ---------------------- | -------------------------------- |
| `orientation` | `SeparatorOrientation` | `SeparatorOrientationHorizontal` |
| `size`        | `Responsive<1..4>`     | `1`                              |
| `color`       | `AccentColor`          | `AccentColor::Gray`              |
| `decorative`  | `bool`                 | `true`                           |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size of the separator. The largest step takes full width or height of the container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["separator"]
files = ["src/separator/separator_size_horizontal.rs"]
show_files = true
url_fragment = "#/size-horizontal"
```

{{#endtab }}
{{#endtabs }}

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["separator"]
files = ["src/separator/separator_size_vertical.rs"]
show_files = true
url_fragment = "#/size-vertical"
```

{{#endtab }}
{{#endtabs }}

### Color

Use the `color` prop to assign a specific [color](../theme/color.md).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["separator"]
files = ["src/separator/separator_color.rs"]
show_files = true
url_fragment = "#/color"
```

{{#endtab }}
{{#endtabs }}

### Orientation

Use the `orientation` prop to control whether the separator is horizontal or vertical.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["separator"]
files = ["src/separator/separator_orientation.rs"]
show_files = true
url_fragment = "#/orientation"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/separator)
