# Strong

Marks text to signify strong importance.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["strong"]
files = ["src/strong/strong.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `strong` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                                 | Default |
| ---------- | ------------------------------------ | ------- |
| `as_child` | `Option<Callback<StrongChildProps>>` | -       |
| `truncate` | `Option<bool>`                       | -       |
| `wrap`     | `Option<TextWrap>`                   | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Truncate

Use the `truncate` prop to truncate text with an ellipsis when it overflows its container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["strong-truncate"]
files = ["src/strong/strong_truncate.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/strong)
