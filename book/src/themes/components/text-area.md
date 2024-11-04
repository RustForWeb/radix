# Text Area

Captures multi-line user input.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-area"]
files = ["src/text_area/text_area.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `textarea` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop      | Type                         | Default                     |
| --------- | ---------------------------- | --------------------------- |
| `size`    | `Responsive<1..3>`           | `2`                         |
| `variant` | `TextFieldVariant`           | `TextFieldVariant::Surface` |
| `resize`  | `Responsive<TextAreaResize>` | -                           |
| `color`   | `Option<AccentColor>`        | -                           |
| `radius`  | `Option<Radius>`             | -                           |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-area"]
files = ["src/text_area/text_area_size.rs"]
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
features = ["text-area"]
files = ["src/text_area/text_area_variant.rs"]
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
features = ["text-area"]
files = ["src/text_area/text_area_color.rs"]
show_files = true
url_fragment = "#/color"
```

{{#endtab }}
{{#endtabs }}

### Radius

Use the `radius` prop to assign a specific radius value.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-area"]
files = ["src/text_area/text_area_radius.rs"]
show_files = true
url_fragment = "#/radius"
```

{{#endtab }}
{{#endtabs }}

### Radius

Use the `resize` prop to enable resizing on one or both axis.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-area"]
files = ["src/text_area/text_area_resize.rs"]
show_files = true
url_fragment = "#/resize"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/text-area)
