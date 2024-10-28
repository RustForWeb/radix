# Kbd

Represents keyboard input or a hotkey.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["kbd"]
files = ["src/kbd/kbd.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `kbd` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                              | Default |
| ---------- | --------------------------------- | ------- |
| `as_child` | `Option<Callback<KbdChildProps>>` | -       |
| `size`     | `Option<Responsive<1..9>>`        | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control text size.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["kbd-size"]
files = ["src/kbd/kbd_size.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/kbd)
