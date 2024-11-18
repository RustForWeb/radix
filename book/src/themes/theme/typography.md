# Typography

Guidance for using and tuning typography.

## Base Components

Use [Text](../typography/text.md) and [Heading](../typography/heading.md) components to render titles and body copy. These components share `size` and `weight` props and map to the underlying type system to ensure consistent typography throughout your app.

TODO: example

## Formatting Components

Compose formatting components to add [emphasis](../typography/em.md), [signal importance](../typography/strong.md), present [code](../typography/code.md) and more.

TODO: example

## Type Scale

The typographic system is based on a 9-step `size` scale. Every step has a corresponding font size, line height and letter spacing value which are all designed to be used in combination.

TODO: visual
TODO: example

| Step | Size   | Letter Spacing | Line Height |
| ---- | ------ | -------------- | ----------- |
| `1`  | `12px` | `0.0025em`     | `16px`      |
| `2`  | `14px` | `0em`          | `20px`      |
| `3`  | `16px` | `0em`          | `24px`      |
| `4`  | `18px` | `-0.0025em`    | `26px`      |
| `5`  | `20px` | `-0.005em`     | `28px`      |
| `6`  | `24px` | `-0.00625em`   | `30px`      |
| `7`  | `28px` | `-0.0075em`    | `36px`      |
| `8`  | `35px` | `-0.01em`      | `40px`      |
| `9`  | `60px` | `-0.025em`     | `60px`      |

## Font Weight

The following weights are supported. Weights can be [customized](#custom-font-weights) to map to your own typeface.

TODO: example

| Weight            | Description | Default Value |
| ----------------- | ----------- | ------------- |
| `Weight::Light`   | Light       | `300`         |
| `Weight::Regular` | Regular     | `400`         |
| `Weight::Medium`  | Medium      | `500`         |
| `Weight::Bold`    | Bold        | `700`         |

## Font Family

By default, Radix Themes uses a system font stack for portability and legibility. Continue to the next section to learn about customizing your theme with a custom font.

| Type     | Default value                                                                                                                                                          |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Text     | `-apple-system, BlinkMacSystemFont, 'Segoe UI (Custom)', Roboto, 'Helvetica Neue', 'Open Sans (Custom)', system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'` |
| Code     | `'Menlo', 'Consolas (Custom)', 'Bitstream Vera Sans Mono', monospace, 'Apple Color Emoji', 'Segoe UI Emoji'`                                                           |
| Emphasis | `'Times New Roman', 'Times', serif`                                                                                                                                    |
| Quote    | `'Times New Roman', 'Times', serif`                                                                                                                                    |

## Customization

Radix Themes typography can be customized by overriding the corresponding CSS variables of the token system. Refer to the [source code](https://github.com/radix-ui/themes/blob/main/packages/radix-ui-themes/src/styles/tokens/typography.css) for the full list of the typographic tokens.

Make sure that your CSS is applied after the Radix Themes styles so that it takes precedence.

### Custom Fonts

You can provide your own fonts, however, how you choose to import and serve them is up to you. It is only required that you specify your named fonts using the correct syntax.

To customize the font family used in your theme, remap the corresponding `font-family` tokens:

```css
.radix-themes {
    --default-font-family: /* Your custom default font */ '';
    --heading-font-family: /* Your custom font for <Heading> components */ '';
    --code-font-family:  /* Your custom font for <Code> components */ '';
    --strong-font-family:  /* Your custom font for <Strong> components */ '';
    --em-font-family:  /* Your custom font for <Em> components */ '';
    --quote-font-family:  /* Your custom font for <Quote> components */ '';
}
```

### Custom Font Weights

To customize the exact font weights used in your theme, remap the corresponding `font-weight` tokens to your own values:

```css
.radix-themes {
    --font-weight-light: 200;
    --font-weight-regular: 300;
    --font-weight-medium: 600;
    --font-weight-bold: 800;
}
```

### Advanced Settings

There's a number of advanced typographic features that can be customized. These include a font size multiplier for certain components, font style, letter spacing, and leading trim. For example, you can customize the headings to render with a relatively larger font size than your default font, different leading trim values, and tighter letter spacing:

```css
.radix-themes {
    --heading-font-family: 'Untitled Sans', sans-serif;
    --heading-font-size-adjust: 1.05;
    --heading-font-style: normal;
    --heading-leading-trim-start: 0.42em;
    --heading-leading-trim-end: 0.38em;
    --heading-letter-spacing: -0.01em;
}
```

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/theme/typography)
