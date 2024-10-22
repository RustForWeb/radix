# Grid

Component for creating grid layouts.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["grid"]
files = ["src/grid/grid.rs"]
```

{{#endtab }}
{{#endtabs }}

## API Reference

TODO

## Examples

### Responsive

All props marked `Responsive`, such as `columns` and `rows` accept a [breakpoint struct instance](../theme/breakpoints.md). For example, the following grid starts with 1 column, and uses 2 columns from the medium breakpoint.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["grid-responsive"]
files = ["src/grid/grid_responsive.rs"]
```

{{#endtab }}
{{#endtabs }}
