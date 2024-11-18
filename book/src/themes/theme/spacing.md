# Spacing

Overview of the space scale and scaling options.

## Space scale

Spacing values are derived from a 9-step scale, which is used for props such as margin and padding. These props accept numeric values from `1` to `9`, which correspond to the steps in the scale below.

TODO: visual

| Step | Base value |
| ---- | ---------- |
| `1`  | `4px`      |
| `2`  | `8px`      |
| `3`  | `12px`     |
| `4`  | `16px`     |
| `5`  | `24px`     |
| `6`  | `32px`     |
| `7`  | `40px`     |
| `8`  | `48px`     |
| `9`  | `64px`     |

### Space Scale Tokens

Space scale tokens can be also accessed using CSS variables. You can use these tokens to style your custom components, ensuring they are consistent with the rest of your theme.

```css
/* Space scale */
var(--space-1);
var(--space-2);
var(--space-3);
var(--space-4);
var(--space-5);
var(--space-6);
var(--space-7);
var(--space-8);
var(--space-9);
```

## Scaling

Values which affect layout (spacing, font size, line height) scale relatively to each other based on the `scaling` value defined in your [Theme](../utilities/theme.md). This setting allows you to scale the UI density uniformly across your entire application.

TODO: example

### Scaling Factor

The scaling factor token can be accessed using the `--scaling` CSS variable. If you need to use different scaling factors in your app, you can use this token in your custom styles, ensuring they are consistent with the rest of your theme.

```css
.MyCustomComponent {
    width: calc(200px * var(--scaling));
}
```

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/theme/spacing)
