# Link

Semantic element for navigation between pages.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["link"]
files = ["src/link/link.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `a` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                               | Default               |
| --------------- | ---------------------------------- | --------------------- |
| `as_child`      | `Option<Callback<LinkChildProps>>` | -                     |
| `size`          | `Option<Responsive<1..9>>`         | -                     |
| `weight`        | `Option<Responsive<Weight>>`       | -                     |
| `trim`          | `Option<Responsive<LeadingTrim>>`  | -                     |
| `truncate`      | `Option<bool>`                     | -                     |
| `wrap`          | `Option<Responsive<TextWrap>>`     | -                     |
| `underline`     | `LinkUnderline`                    | `LinkUnderline::Auto` |
| `color`         | `Option<AccentColor>`              | -                     |
| `high_contrast` | `Option<bool>`                     | -                     |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size of the link. The prop also provides correct line height and corrective letter spacing - as text size increases, the relative line height and letter spacing decrease.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["link"]
files = ["src/link/link_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Weight

Use the `weight` prop to set the text weight.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["link"]
files = ["src/link/link_weight.rs"]
show_files = true
url_fragment = "#/weight"
```

{{#endtab }}
{{#endtabs }}

### Truncate

Use the `truncate` prop to truncate text with an ellipsis when it overflows its container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["link"]
files = ["src/link/link_truncate.rs"]
show_files = true
url_fragment = "#/truncate"
```

{{#endtab }}
{{#endtabs }}

### Color

Use the `color` prop to assign a specific [color](../theme/color.md).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["link"]
files = ["src/link/link_color.rs"]
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
features = ["link"]
files = ["src/link/link_high_contrast.rs"]
show_files = true
url_fragment = "#/high-contrast"
```

{{#endtab }}
{{#endtabs }}

### Underline

Use the `underline` prop to manage the visibility of the underline affordance.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["link"]
files = ["src/link/link_underline.rs"]
show_files = true
url_fragment = "#/underline"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/link)
