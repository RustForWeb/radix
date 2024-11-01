# Icon Button

Button designed specifically for usage with a single icon.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["icon-button"]
files = ["src/icon_button/icon_button.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `button` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                                     | Default                    |
| --------------- | ---------------------------------------- | -------------------------- |
| `as_child`      | `Option<Callback<IconButtonChildProps>>` | -                          |
| `size`          | `Responsive<1..4>`                       | `2`                        |
| `variant`       | `IconButtonVariant`                      | `IconButtonVariant::Solid` |
| `color`         | `Option<AccentColor>`                    | -                          |
| `high_contrast` | `Option<bool>`                           | -                          |
| `radius`        | `Option<Radius>`                         | -                          |
| `loading`       | `bool`                                   | `false`                    |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size of the button.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["icon-button"]
files = ["src/icon_button/icon_button_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style of the button.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["icon-button"]
files = ["src/icon_button/icon_button_variant.rs"]
show_files = true
url_fragment = "#/variant"
```

{{#endtab }}
{{#endtabs }}

#### Ghost

Use the `ghost` variant to display a button without chrome. Ghost buttons behave like text in layout, as they use a negative margin to optically align themselves against their siblings while maintaining the padding in active and hover states.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["icon-button"]
files = ["src/icon_button/icon_button_variant_ghost.rs"]
show_files = true
url_fragment = "#/variant-ghost"
```

{{#endtab }}
{{#endtabs }}

### Color

Use the `color` prop to assign a specific [color](../theme/color.md).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["icon-button"]
files = ["src/icon_button/icon_button_color.rs"]
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
features = ["icon-button"]
files = ["src/icon_button/icon_button_high_contrast.rs"]
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
features = ["icon-button"]
files = ["src/icon_button/icon_button_radius.rs"]
show_files = true
url_fragment = "#/radius"
```

{{#endtab }}
{{#endtabs }}

### Loading

Use the `loading` prop to display a loading spinner in place of button content. The button will be disabled while loading.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["icon-button"]
files = ["src/icon_button/icon_button_loading.rs"]
show_files = true
url_fragment = "#/loading"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/icon-button)
