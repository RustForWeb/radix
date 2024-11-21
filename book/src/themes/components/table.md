# Table

Semantic table element for presenting data.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["table"]
files = ["src/table/table.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Root

Groups the `Header` and `Body` parts. This component is based on the `table` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop      | Type                              | Default               |
| --------- | --------------------------------- | --------------------- |
| `size`    | `Responsive<1..3>`                | `1`                   |
| `variant` | `TableVariant`                    | `TableVariant::Ghost` |
| `layout`  | `Option<Responsive<TableLayout>>` | -                     |

{{#endtab }}
{{#endtabs }}

### Header

Contains the column headings for the table, based on the `thead` element.

### Body

Displays the table data. This component is based on the `tbody` element.

### Row

A row of table cells. Based on the `tr` element.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop    | Type                                | Default |
| ------- | ----------------------------------- | ------- |
| `align` | `Option<Responsive<TableRowAlign>>` | -       |

{{#endtab }}
{{#endtabs }}

### Cell

A basic table cell. This component is based on the `td` element, but uses `justify` instead of align to control how horizontal space is distributed within the table cell.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop        | Type                                   | Default |
| ----------- | -------------------------------------- | ------- |
| `justify`   | `Option<Responsive<TableCellJustify>>` | -       |
| `width`     | `Option<Responsive<String>>`           | -       |
| `min_width` | `Option<Responsive<String>>`           | -       |
| `max_width` | `Option<Responsive<String>>`           | -       |
| `p`         | `Option<Responsive<0..9>>`             | -       |
| `px`        | `Option<Responsive<0..9>>`             | -       |
| `py`        | `Option<Responsive<0..9>>`             | -       |
| `pt`        | `Option<Responsive<0..9>>`             | -       |
| `pr`        | `Option<Responsive<0..9>>`             | -       |
| `pb`        | `Option<Responsive<0..9>>`             | -       |
| `pl`        | `Option<Responsive<0..9>>`             | -       |

{{#endtab }}
{{#endtabs }}

### ColumnHeaderCell

The header of a table column. Based on the `th` element and provides the same props interface as the `Cell` part.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop        | Type                                   | Default |
| ----------- | -------------------------------------- | ------- |
| `justify`   | `Option<Responsive<TableCellJustify>>` | -       |
| `width`     | `Option<Responsive<String>>`           | -       |
| `min_width` | `Option<Responsive<String>>`           | -       |
| `max_width` | `Option<Responsive<String>>`           | -       |
| `p`         | `Option<Responsive<0..9>>`             | -       |
| `px`        | `Option<Responsive<0..9>>`             | -       |
| `py`        | `Option<Responsive<0..9>>`             | -       |
| `pt`        | `Option<Responsive<0..9>>`             | -       |
| `pr`        | `Option<Responsive<0..9>>`             | -       |
| `pb`        | `Option<Responsive<0..9>>`             | -       |
| `pl`        | `Option<Responsive<0..9>>`             | -       |

{{#endtab }}
{{#endtabs }}

### RowHeaderCell

The header of a table row. Based on the `th` element and provides the same props interface as the `Cell` part.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop        | Type                                   | Default |
| ----------- | -------------------------------------- | ------- |
| `justify`   | `Option<Responsive<TableCellJustify>>` | -       |
| `width`     | `Option<Responsive<String>>`           | -       |
| `min_width` | `Option<Responsive<String>>`           | -       |
| `max_width` | `Option<Responsive<String>>`           | -       |
| `p`         | `Option<Responsive<0..9>>`             | -       |
| `px`        | `Option<Responsive<0..9>>`             | -       |
| `py`        | `Option<Responsive<0..9>>`             | -       |
| `pt`        | `Option<Responsive<0..9>>`             | -       |
| `pr`        | `Option<Responsive<0..9>>`             | -       |
| `pb`        | `Option<Responsive<0..9>>`             | -       |
| `pl`        | `Option<Responsive<0..9>>`             | -       |

{{#endtab }}
{{#endtabs }}

## Examples

### Size

Use the `size` prop to control how large the text and padding of the table cells should be.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["table"]
files = ["src/table/table_size.rs"]
show_files = true
url_fragment = "#/size"
```

{{#endtab }}
{{#endtabs }}

### With a Backplate

Use `TableVariant::Surface` to add a visually enclosed backplate to the table.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["table"]
files = ["src/table/table_with_a_backplate.rs"]
show_files = true
url_fragment = "#/with-a-backplate"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/table)
