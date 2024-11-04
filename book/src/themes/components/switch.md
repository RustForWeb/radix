# Switch

Toggle switch alternative to the checkbox.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["switch"]
files = ["src/switch/switch.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component inherits props from the [Switch primitive](../../primitives/components/switch.md) and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                  | Default                  |
| --------------- | --------------------- | ------------------------ |
| `size`          | `Responsive<1..3>`    | `2`                      |
| `variant`       | `SwitchVariant`       | `SwitchVariant::Surface` |
| `color`         | `Option<AccentColor>` | -                        |
| `high_contrast` | `Option<bool>`        | -                        |
| `radius`        | `Option<Radius>`      | -                        |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size of the switch.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["switch"]
files = ["src/switch/switch_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style of the switch.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["switch"]
files = ["src/switch/switch_variant.rs"]
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
features = ["switch"]
files = ["src/switch/switch_color.rs"]
show_files = true
url_fragment = "#/color"
```

{{#endtab }}
{{#endtabs }}

### High-Contrast

Use the `high_contrast` prop to increase color contrast in light mode.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["switch"]
files = ["src/switch/switch_high_contrast.rs"]
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
features = ["switch"]
files = ["src/switch/switch_radius.rs"]
show_files = true
url_fragment = "#/radius"
```

{{#endtab }}
{{#endtabs }}

### Alignment

Composing `Switch` within `Text` automatically centers it with the first line of text.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["switch"]
files = ["src/switch/switch_alignment.rs"]
show_files = true
url_fragment = "#/alignment"
```

{{#endtab }}
{{#endtabs }}

It is automatically well-aligned with multi-line text too.

### Disabled

Use the native `disabled` attribute to create a disabled switch.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["switch"]
files = ["src/switch/switch_disabled.rs"]
show_files = true
url_fragment = "#/disabled"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/switch)
