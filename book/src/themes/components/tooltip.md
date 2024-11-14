# Tooltip

Floating element that provides a control with contextual information via pointer or focus.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["tooltip"]
files = ["src/tooltip/tooltip.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component inherits and merges props from the Radix Tooltip primitive [Root](../../primitives/components/tooltip.md#root), [Portal](../../primitives/components/tooltip.md#portal) and [Content](../../primitives/components/tooltip.md#content) parts. It supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop        | Type                         | Default |
| ----------- | ---------------------------- | ------- |
| `content`   | `Html`                       | -       |
| `width`     | `Option<Responsive<String>>` | -       |
| `min_width` | `Option<Responsive<String>>` | -       |
| `max_width` | `Responsive<String>`         | `360p`  |

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/tooltip)
