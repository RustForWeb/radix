# Code

Marks text to signify a short fragment of computer code.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["code"]
files = ["src/code/code.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `code` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                               | Default             |
| --------------- | ---------------------------------- | ------------------- |
| `as_child`      | `Option<Callback<CodeChildProps>>` | -                   |
| `size`          | `Option<Responsive<1..9>>`         | -                   |
| `variant`       | `CodeVariant`                      | `CodeVariant::Soft` |
| `weight`        | `Option<Responsive<Weight>>`       | -                   |
| `color`         | `Option<AccentColor>`              | -                   |
| `high_contrast` | `Option<bool>`                     | -                   |
| `truncate`      | `Option<bool>`                     | -                   |
| `wrap`          | `Option<Responsive<TextWrap>>`     | -                   |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control text size. This prop also provides correct line height and corrective letter spacing - as text size increases, the relative line height and letter spacing decrease.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["code-size"]
files = ["src/code/code_size.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["code-variant"]
files = ["src/code/code_variant.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Color

Use the `color` prop to assign a specific [color](../theme/color.md).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["code-color"]
files = ["src/code/code_color.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### High-Contrast

Use the `high_contrast` prop to increase color contrast with the background.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["code-high-contrast"]
files = ["src/code/code_high_contrast.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Weight

Use the `weight` prop to set the text weight.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["code-weight"]
files = ["src/code/code_weight.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Truncate

Use the `truncate` prop to truncate text with an ellipsis when it overflows its container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["code-truncate"]
files = ["src/code/code_truncate.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/code)
