# Text

Foundational text primitive.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text"]
files = ["src/text/text.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## API Reference

TODO

## Examples

### As Another Element

Use the `as` prop to render text as a `p`, `label`, `div` or `span`. This prop is purely semantic and does not alter visual appearance.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-as"]
files = ["src/text/text_as.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

### Size

Use the `size` prop to control text size. This prop also provides correct line height and corrective letter spacing - as text size increases, the relative line height and letter spacing decrease.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-size"]
files = ["src/text/text_size.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

Sizes 2-4 are designed to work well for long-form content.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-size-content"]
files = ["src/text/text_size_content.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

Sizes 1-3 are designed to work well for UI labels.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-size-labels"]
files = ["src/text/text_size_labels.rs"]
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
features = ["text-weight"]
files = ["src/text/text_weight.rs"]
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
features = ["text-align"]
files = ["src/text/text_align.rs"]
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
features = ["text-trim"]
files = ["src/text/text_trim.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

Trimming the leading is useful when dialing in vertical spacing in cards or other "boxy" components. Otherwise, padding looks larger on top and bottom than on the sides.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-trim-box"]
files = ["src/text/text_trim_box.rs"]
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
features = ["text-truncate"]
files = ["src/text/text_truncate.rs"]
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
features = ["text-wrap"]
files = ["src/text/text_wrap.rs"]
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
features = ["text-color"]
files = ["src/text/text_color.rs"]
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
features = ["text-high-contrast"]
files = ["src/text/text_high_contrast.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## With Formatting

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-formatting"]
files = ["src/text/text_formatting.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## With Form Controls

Compose `Text` with formatting components to add emphasis and structure to content.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

Composing `Text` with a form control like [`Checkbox`](../components/checkbox.md), [`RadioGroup`](../components/radio-group.md), or [`Switch`](../components/switch.md) automatically centers the control with the first line of text, even when the text is multi-line.

```toml,trunk
package = "radix-yew-book-themes"
features = ["text-form-controls"]
files = ["src/text/text_form_controls.rs"]
show_files = true
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/components/text)
