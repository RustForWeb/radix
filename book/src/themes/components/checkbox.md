# Checkbox

Base input element to toggle an option on and off.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["checkbox"]
files = ["src/checkbox/checkbox.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component inherits props from the [Checkbox primitive](../../primitives/components/checkbox.md) and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                  | Default                    |
| --------------- | --------------------- | -------------------------- |
| `size`          | `Responsive<1..3>`    | `2`                        |
| `variant`       | `CheckboxVariant`     | `CheckboxVariant::Surface` |
| `color`         | `Option<AccentColor>` | -                          |
| `high_contrast` | `Option<bool>`        | -                          |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size of the checkbox.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["checkbox"]
files = ["src/checkbox/checkbox_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style of the checkbox.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["checkbox"]
files = ["src/checkbox/checkbox_variant.rs"]
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
features = ["checkbox"]
files = ["src/checkbox/checkbox_color.rs"]
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
features = ["checkbox"]
files = ["src/checkbox/checkbox_high_contrast.rs"]
show_files = true
url_fragment = "#/high-contrast"
```

{{#endtab }}
{{#endtabs }}

### Alignment

Composing `Checkbox` within `Text` automatically centers it with the first line of text.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["checkbox"]
files = ["src/checkbox/checkbox_alignment.rs"]
show_files = true
url_fragment = "#/alignment"
```

{{#endtab }}
{{#endtabs }}

It is automatically well-aligned with multi-line text too.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["checkbox"]
files = ["src/checkbox/checkbox_alignment_multi_line.rs"]
show_files = true
url_fragment = "#/alignment-multi-line"
```

{{#endtab }}
{{#endtabs }}

### Disabled

Use the native `disabled` attribute to create a disabled checkbox.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["checkbox"]
files = ["src/checkbox/checkbox_disabled.rs"]
show_files = true
url_fragment = "#/disabled"
```

{{#endtab }}
{{#endtabs }}

### Disabled

Use the `CheckedState::Indeterminate` value to create an indeterminate checkbox.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["checkbox"]
files = ["src/checkbox/checkbox_indeterminate.rs"]
show_files = true
url_fragment = "#/indeterminate"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/checkbox)
