# Inset

Applies a negative margin to allow content to bleed into the surrounding container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["inset"]
files = ["src/inset/inset.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `div` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                                | Default                |
| ---------- | ----------------------------------- | ---------------------- |
| `as_child` | `Option<Callback<InsetChildProps>>` | -                      |
| `side`     | `Responsive<InsetSide>`             | `InsetSide::All`       |
| `clip`     | `Responsive<InsetClip>`             | `InsetClip::BorderBox` |
| `p`        | `Option<Responsive<InsetPadding>>`  | -                      |
| `px`       | `Option<Responsive<InsetPadding>>`  | -                      |
| `py`       | `Option<Responsive<InsetPadding>>`  | -                      |
| `pt`       | `Option<Responsive<InsetPadding>>`  | -                      |
| `pr`       | `Option<Responsive<InsetPadding>>`  | -                      |
| `pb`       | `Option<Responsive<InsetPadding>>`  | -                      |
| `pl`       | `Option<Responsive<InsetPadding>>`  | -                      |

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/inset)
