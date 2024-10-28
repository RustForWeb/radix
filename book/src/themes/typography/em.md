# Em

Marks text to stress emphasis.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["em"]
files = ["src/em/em.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `em` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                             | Default |
| ---------- | -------------------------------- | ------- |
| `as_child` | `Option<Callback<EmChildProps>>` | -       |
| `truncate` | `Option<bool>`                   | -       |
| `wrap`     | `Option<TextWrap>`               | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Truncate

Use the `truncate` prop to truncate text with an ellipsis when it overflows its container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["em-truncate"]
files = ["src/em/em_truncate.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/em)
