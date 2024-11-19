# Callout

Short message to attract user's attention.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["callout"]
files = ["src/callout/callout.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Groups Icon and Text parts. This component is based on the `div` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                                  | Default                |
| --------------- | ------------------------------------- | ---------------------- |
| `as_child`      | `Option<Callback<CalloutChildProps>>` | -                      |
| `size`          | `Responsive<1..3>`                    | `1`                    |
| `variant`       | `CalloutVariant`                      | `CalloutVariant::Soft` |
| `color`         | `Option<AccentColor>`                 | -                      |
| `high_contrast` | `Option<bool>`                        | -                      |

{{#endtab }}
{{#endtabs }}

### Icon

Provides width and height for the icon associated with the callout. This component is based on the `div` element.

### Text

Renders the callout text. This component is based on the `p` element.

## Examples

### Size

Use the `size` prop to control the size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["callout"]
files = ["src/callout/callout_size.rs"]
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
features = ["callout"]
files = ["src/callout/callout_variant.rs"]
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
features = ["callout"]
files = ["src/callout/callout_color.rs"]
show_files = true
url_fragment = "#/color"
```

{{#endtab }}
{{#endtabs }}

### High-Contrast

Use the `high_contrast` prop to increase color contrast.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["callout"]
files = ["src/callout/callout_high_contrast.rs"]
show_files = true
url_fragment = "#/high-contrast"
```

{{#endtab }}
{{#endtabs }}

### As Alert

Add a native [WAI-ARIA `alert` role](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/alert_role) to the callout when the user's immediate attention is required, like when an error message appears. The screen reader will be interrupted, announcing the new content immediately.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["callout"]
files = ["src/callout/callout_as_alert.rs"]
show_files = true
url_fragment = "#/as-alert"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/callout)
