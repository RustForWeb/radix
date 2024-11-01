# Grid

Component for creating grid layouts.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["grid"]
files = ["src/grid/grid.rs"]
show_files = true
url_fragment = "#/"
```

{{#endtab }}
{{#endtabs }}

## API Reference

TODO

## Examples

### Responsive

All props marked `Responsive`, such as `columns` and `rows` accept a [breakpoint values](../theme/breakpoints.md). For example, the following grid starts with 1 column, and uses 2 columns from the medium breakpoint.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["grid"]
files = ["src/grid/grid_responsive.rs"]
show_files = true
url_fragment = "#/responsive"
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/grid)
