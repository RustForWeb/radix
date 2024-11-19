# Card

Container that groups related content and actions.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["card"]
files = ["src/card/card.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `div` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                               | Default                |
| ---------- | ---------------------------------- | ---------------------- |
| `as_child` | `Option<Callback<CardChildProps>>` | -                      |
| `size`     | `Responsive<1..5>`                 | `1`                    |
| `variant`  | `CardVariant`                      | `CardVariant::Surface` |

{{#endtab }}
{{#endtabs }}

## Examples

### As Another Element

Use the `as_child` prop to render the card as a link or a button. This prop adds styles for the interactive states, like hover and focus.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["card"]
files = ["src/card/card_as_another_element.rs"]
show_files = true
url_fragment = "#/as-another-element"
```

{{#endtab }}
{{#endtabs }}

### Size

Use the `size` prop to control the size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["card"]
files = ["src/card/card_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["card"]
files = ["src/card/card_variant.rs"]
show_files = true
url_fragment = "#/variant"
```

{{#endtab }}
{{#endtabs }}

### With Inset Content

Use the [Inset](./inset.md) component to align content flush with the sides of the card.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["card"]
files = ["src/card/card_with_inset_content.rs"]
show_files = true
url_fragment = "#/with-inset-content"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/card)
