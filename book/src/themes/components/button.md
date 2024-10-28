# Button

Trigger an action or event, such as submitting a form or displaying a dialog.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button"]
files = ["src/button/button.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `button` element and supports [common margin props](../overview/layout.md#margin-props).

| Prop            | Type                                 | Default                |
| --------------- | ------------------------------------ | ---------------------- |
| `as_child`      | `Option<Callback<ButtonChildProps>>` | -                      |
| `size`          | `Responsive<1..4>`                   | `2`                    |
| `variant`       | `ButtonVariant`                      | `ButtonVariant::Solid` |
| `color`         | `AccentColor`                        | -                      |
| `high_contrast` | `Option<bool>`                       | -                      |
| `radius`        | `Option<Radius>`                     | -                      |
| `loading`       | `bool`                               | `false`                |

## Examples

### Size

Use the `size` prop to control the size of the button.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button-size"]
files = ["src/button/button_size.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style of the button.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button-variant"]
files = ["src/button/button_variant.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

#### Ghost

Use the `ghost` variant to display a button without chrome. Ghost buttons behave like text in layout, as they use a negative margin to optically align themselves against their siblings while maintaining the padding in active and hover states.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button-variant-ghost"]
files = ["src/button/button_variant_ghost.rs"]
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
features = ["button-color"]
files = ["src/button/button_color.rs"]
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
features = ["button-high-contrast"]
files = ["src/button/button_high_contrast.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Radius

Use the `radius` prop to assign a specific radius value.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button-radius"]
files = ["src/button/button_radius.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### With Icons

You can nest icons directly inside the button. An appropriate gap is provided automatically.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button-with-icons"]
files = ["src/button/button_with_icons.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Loading

Use the `loading` prop to display a loading spinner in place of button content, preserving the original size of the button in its normal state. The button will be disabled while loading.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button-loading"]
files = ["src/button/button_loading.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

If you have an icon inside the button, you can use the button's `disabled` state and wrap the icon in a standalone [Spinner](./spinner.md) to achieve a more sophisticated design.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["button-loading-spinner"]
files = ["src/button/button_loading_spinner.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/button)
