# Radio

Standalone radio button that can be used in any layout.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["radio"]
files = ["src/radio/radio.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component inherits props from the [Radio Group primitive](../../primitives/components/radio-group.md) element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                  | Default                 |
| --------------- | --------------------- | ----------------------- |
| `size`          | `Responsive<1..3>`    | `2`                     |
| `variant`       | `RadioVariant`        | `RadioVariant::Surface` |
| `color`         | `Option<AccentColor>` | -                       |
| `high_contrast` | `Option<bool>`        | -                       |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the radio button size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["radio"]
files = ["src/radio/radio_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style of the radio buttons.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["radio"]
files = ["src/radio/radio_variant.rs"]
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
features = ["radio"]
files = ["src/radio/radio_color.rs"]
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
features = ["radio"]
files = ["src/radio/radio_high_contrast.rs"]
show_files = true
url_fragment = "#/high-contrast"
```

{{#endtab }}
{{#endtabs }}

### Alignment

Composing `Radio` within `Text` automatically centers it with the first line of text. It is automatically well-aligned with multi-line text too.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["radio"]
files = ["src/radio/radio_alignment.rs"]
show_files = true
url_fragment = "#/alignment"
```

{{#endtab }}
{{#endtabs }}

### Disabled

Use the native `disabled` attribute to create a disabled radio button.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["radio"]
files = ["src/radio/radio_disabled.rs"]
show_files = true
url_fragment = "#/disabled"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/radio)
