# Blockquote

Block-level quotation from another source.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["blockquote"]
files = ["src/blockquote/blockquote.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `blockquote` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                                     | Default |
| --------------- | ---------------------------------------- | ------- |
| `as_child`      | `Option<Callback<BlockquoteChildProps>>` | -       |
| `size`          | `Option<Responsive<1..9>>`               | -       |
| `weight`        | `Option<Responsive<Weight>>`             | -       |
| `color`         | `Option<AccentColor>`                    | -       |
| `high_contrast` | `Option<bool>`                           | -       |
| `truncate`      | `Option<bool>`                           | -       |
| `wrap`          | `Option<Responsive<TextWrap>>`           | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["blockquote-size"]
files = ["src/blockquote/blockquote_size.rs"]
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
features = ["blockquote-weight"]
files = ["src/blockquote/blockquote_weight.rs"]
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
features = ["blockquote-color"]
files = ["src/blockquote/blockquote_color.rs"]
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
features = ["blockquote-high-contrast"]
files = ["src/blockquote/blockquote_high_contrast.rs"]
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
features = ["blockquote-truncate"]
files = ["src/blockquote/blockquote_truncate.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/blockquote)
