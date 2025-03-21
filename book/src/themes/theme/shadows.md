# Shadows

Understanding the shadow styles used in your theme.

Shadows used in the components are derived from a 6-step scale. Refer to the [source code](https://github.com/radix-ui/themes/blob/main/packages/radix-ui-themes/src/styles/tokens/shadow.css) for the actual values used to achieve these shadows.

TODO: visual

## Shadow Tokens

Shadow tokens can be accessed using CSS variables. You can use these tokens to style your custom components, ensuring they are consistent with the rest of your theme.

```css
/* Inset shadow */
var(--shadow-1);

/* Shadows for variant="classic" panels, like Card */
var(--shadow-2);
var(--shadow-3);

/* Shadows for smaller overlay panels, like Hover Card and Popover */
var(--shadow-4);
var(--shadow-5);

/* Shadows for larger overlay panels, like Dialog */
var(--shadow-6);
```

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/theme/shadows)
