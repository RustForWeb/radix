# Data List

Displays metadata as a list of key-value pairs.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["data-list"]
files = ["src/data_list/data_list.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `dl` element and supports [common margin props](../overview/layout.md#margin-props).

### Root

Contains all the parts of the data list.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop          | Type                               | Default                           |
| ------------- | ---------------------------------- | --------------------------------- |
| `orientation` | `Responsive<DataListOrientation>`  | `DataListOrientation::Horizontal` |
| `size`        | `Responsive<1..3>`                 | `2`                               |
| `trim`        | `Option<Responsive<DataListTrim>>` | -                                 |

{{#endtab }}
{{#endtabs }}

### Item

Wraps a key-value pair.

| Prop    | Type                                    | Default |
| ------- | --------------------------------------- | ------- |
| `align` | `Option<Responsive<DataListItemAlign>>` | -       |

### Label

Contains the key of the key-value pair.

| Prop            | Type                         | Default |
| --------------- | ---------------------------- | ------- |
| `width`         | `Option<Responsive<String>>` | -       |
| `min_width`     | `Option<Responsive<String>>` | -       |
| `max_width`     | `Option<Responsive<String>>` | -       |
| `color`         | `Option<AccentColor>`        | -       |
| `high_contrast` | `Option<bool>`               | -       |

### Value

Contains the value of the key-value pair.

## Examples

### Orientation

Use the `orientation` prop to change the way the data list is layed-out.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["data-list"]
files = ["src/data_list/data_list_orientation.rs"]
show_files = true
url_fragment = "#/orientation"
```

{{#endtab }}
{{#endtabs }}

### Size

Use the `size` prop to change the size of the data list.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["data-list"]
files = ["src/data_list/data_list_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### Color

Use the `color` prop on the Label part to assign a specific [color](../theme/color.md).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["data-list"]
files = ["src/data_list/data_list_color.rs"]
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
features = ["data-list"]
files = ["src/data_list/data_list_high_contrast.rs"]
show_files = true
url_fragment = "#/high-contrast"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/data-list)
