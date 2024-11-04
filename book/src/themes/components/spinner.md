# Spinner

Displays an animated loading indicator.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["spinner"]
files = ["src/spinner/spinner.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `span` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop      | Type               | Default |
| --------- | ------------------ | ------- |
| `size`    | `Responsive<1..3>` | `2`     |
| `loading` | `bool`             | `true`  |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control the size of the spinner.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["spinner"]
files = ["src/spinner/spinner_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### With Children

Use the § prop to control whether the spinner or its children are displayed. Spinner preserves the dimensions of children when they are hidden and disables interactive elements.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["spinner"]
files = ["src/spinner/spinner_with_children.rs"]
show_files = true
url_fragment = "#/with-children"
```

{{#endtab }}
{{#endtabs }}

### With Buttons

[Buttons](./button.md) have their own `loading` prop that automatically composes a spinner.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["spinner"]
files = ["src/spinner/spinner_with_buttons.rs"]
show_files = true
url_fragment = "#/with-buttons"
```

{{#endtab }}
{{#endtabs }}

If you have an icon inside the button, you can use the button's `disabled` state and wrap the icon in a standalone `<Spinner>` to achieve a more sophisticated design.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["spinner"]
files = ["src/spinner/spinner_with_buttons_disabled.rs"]
show_files = true
url_fragment = "#/with-buttons-disabled"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/spinner)
