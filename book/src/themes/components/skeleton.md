# Skeleton

Replaces content with same shape placeholder that indicates a loading state.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["skeleton"]
files = ["src/skeleton/skeleton.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `span` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop         | Type                                   | Default |
| ------------ | -------------------------------------- | ------- |
| `as_child`   | `Option<Callback<SkeletonChildProps>>` | -       |
| `loading`    | `bool`                                 | `true`  |
| `width`      | `Option<Responsive<String>>`           | -       |
| `min_width`  | `Option<Responsive<String>>`           | -       |
| `max_width`  | `Option<Responsive<String>>`           | -       |
| `height`     | `Option<Responsive<String>>`           | -       |
| `min_height` | `Option<Responsive<String>>`           | -       |
| `max_height` | `Option<Responsive<String>>`           | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the width and height props to manually control the size of the skeleton.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["skeleton"]
files = ["src/skeleton/skeleton_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### With Children

Use the `loading` prop to control whether the skeleton or its children are displayed. Skeleton preserves the dimensions of children when they are hidden and disables interactive elements.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["skeleton"]
files = ["src/skeleton/skeleton_with_children.rs"]
show_files = true
url_fragment = "#/with-children"
```

{{#endtab }}
{{#endtabs }}

### With Text

When using Skeleton with text, you'd usually wrap the text node itself rather than the parent element. This ensures that the text is replaced with a placeholder of the same size:

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["skeleton"]
files = ["src/skeleton/skeleton_with_text.rs"]
show_files = true
url_fragment = "#/with-text"
```

{{#endtab }}
{{#endtabs }}

The difference is especially noticeable when wrapping longer paragraphs:

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["skeleton"]
files = ["src/skeleton/skeleton_with_text_paragraph.rs"]
show_files = true
url_fragment = "#/with-text-paragraph"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/skeleton)
