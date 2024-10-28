# Quote

Short inline quotation.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["quote"]
files = ["src/quote/quote.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `q` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                                | Default |
| ---------- | ----------------------------------- | ------- |
| `as_child` | `Option<Callback<QuoteChildProps>>` | -       |
| `truncate` | `Option<bool>`                      | -       |
| `wrap`     | `Option<TextWrap>`                  | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Truncate

Use the `truncate` prop to truncate text with an ellipsis when it overflows its container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["quote-truncate"]
files = ["src/quote/quote_truncate.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/quote)
