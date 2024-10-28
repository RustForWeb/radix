# Heading

Semantic heading element.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading"]
files = ["src/heading/heading.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

This component is based on the `h1` element and supports [common margin props](../overview/layout.md#margin-props).

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                                  | Default         |
| --------------- | ------------------------------------- | --------------- |
| `as`            | `HeadingAs`                           | `HeadingAs::H1` |
| `as_child`      | `Option<Callback<HeadingChildProps>>` | -               |
| `size`          | `Responsive<1..9>`                    | `6`             |
| `weight`        | `Option<Weight>`                      | -               |
| `align`         | `Option<TextAlign>`                   | -               |
| `trim`          | `Option<LeadingTrim>`                 | -               |
| `truncate`      | `Option<bool>`                        | -               |
| `wrap`          | `Option<TextWrap>`                    | -               |
| `color`         | `Option<AccentColor>`                 | -               |
| `high_contrast` | `Option<bool>`                        | -               |

{{#endtab }}
{{#endtabs }}

## Examples

### As Another Element

Use the `as` prop to change the heading level. This prop is purely semantic and does not change the visual appearance.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-as"]
files = ["src/heading/heading_as.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Size

Use the `size` prop to control the size of the heading. The prop also provides correct line height and corrective letter spacing - as text size increases, the relative line height and letter spacing decrease.

Heading sizes match [Text](./text.md) sizes. However, the line heights are set a bit tighter as headings tend to be shorter than running text.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-size"]
files = ["src/heading/heading_size.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Weight

Use the `weight` prop to set the text weight.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-weight"]
files = ["src/heading/heading_weight.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Align

Use the `align` prop to set text alignment.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-align"]
files = ["src/heading/heading_align.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Trim

Use the `trim` prop to trim the leading space at the start, end, or both sides of the text box.

The prop works similarly to the upcoming [half-leading control](https://www.w3.org/TR/css-inline-3/#leading-trim) spec, but uses a common [negative margin workaround](https://seek-oss.github.io/capsize/) under the hood for cross-browser support.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-trim"]
files = ["src/heading/heading_trim.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

Trimming the leading is useful when dialing in vertical spacing in cards or other "boxy" components. Otherwise, padding looks larger on top and bottom than on the sides.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-trim-box"]
files = ["src/heading/heading_trim_box.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

The default trim values are configured for the system font stack that's used by Radix Themes. If you are using custom fonts, you can [adjust](../theme/typography.md) the trim values using the corresponding CSS variables.

```css
.radix-themes {
    --default-leading-trim-start: 0.42em;
    --default-leading-trim-end: 0.36em;
    --heading-leading-trim-start: 0.42em;
    --heading-leading-trim-end: 0.36em;
}
```

### Truncate

Use the `truncate` prop to truncate text with an ellipsis when it overflows its container.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-truncate"]
files = ["src/heading/heading_truncate.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Wrap

Use the `wrap` prop to control text wrapping.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-wrap"]
files = ["src/heading/heading_wrap.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

<div class="warning">

`text-wrap: pretty` is an experimental value that is [not yet supported in all browsers](https://developer.mozilla.org/en-US/docs/Web/CSS/text-wrap#browser_compatibility). However, it can be treated as a progressive enhancement for browsers that do support it.

</div>

### Color

Use the `color` prop to assign a specific [color](../theme/color.md). The text colors are designed to achieve at least Lc 60 APCA contrast over common background colors.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["heading-color"]
files = ["src/heading/heading_color.rs"]
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
features = ["heading-high-contrast"]
files = ["src/heading/heading_high_contrast.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/heading)
