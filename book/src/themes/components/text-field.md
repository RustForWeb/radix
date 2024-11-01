# Text Field

Captures user input with an optional slot for buttons and icons.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-field"]
files = ["src/text_field/text_field.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Groups Slot and Input parts. This component is based on the `div` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop      | Type                  | Default                     |
| --------- | --------------------- | --------------------------- |
| `size`    | `Responsive<1..3>`    | `2`                         |
| `variant` | `TextFieldVariant`    | `TextFieldVariant::Surface` |
| `color`   | `Option<AccentColor>` | -                           |
| `radius`  | `Option<Radius>`      | -                           |

{{#endtab }}
{{#endtabs }}

### Slot

Contains icons or buttons associated with an Input.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop    | Type                        | Default |
| ------- | --------------------------- | ------- |
| `side`  | `Option<TextFieldSlotSide>` | -       |
| `color` | `Option<AccentColor>`       | -       |
| `gap`   | `Option<Responsive<0..9>>`  | -       |
| `px`    | `Option<Responsive<0..9>>`  | -       |
| `pl`    | `Option<Responsive<0..9>>`  | -       |
| `pr`    | `Option<Responsive<0..9>>`  | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-field"]
files = ["src/text_field/text_field_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

Use matching component sizes when composing Text Field with buttons. However, don't use size 1 inputs with buttons - at this size, there is not enough vertical space to nest other interactive elements.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-field"]
files = ["src/text_field/text_field_size_buttons.rs"]
show_files = true
url_fragment = "#/size-buttons"
```

{{#endtab }}
{{#endtabs }}

### Variant

Use the `variant` prop to control the visual style.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-field"]
files = ["src/text_field/text_field_variant.rs"]
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
features = ["text-field"]
files = ["src/text_field/text_field_color.rs"]
show_files = true
url_fragment = "#/color"
```

{{#endtab }}
{{#endtabs }}

### Radius

Use the `radius` prop to assign a specific radius value.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-field"]
files = ["src/text_field/text_field_radius.rs"]
show_files = true
url_fragment = "#/radius"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/text-field)
