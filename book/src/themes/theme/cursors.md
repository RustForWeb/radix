# Cursors

Customizing cursors used for interactive elements.

## Default Cursors

By default, interactive elements that don\t link to another page use the regular arrow cursor. This also matches the browser defaults. However, disabled elements use an explicit disabled cursor.

TODO: example

## Cursor Tokens

Cursor settings can be accessed using CSS variables. You can use these tokens to style your custom components, ensuring they are accessible and consistent with the rest of your theme.

```css
/* Available cursor tokens */
var(--cursor-button);
var(--cursor-checkbox);
var(--cursor-disabled);
var(--cursor-link);
var(--cursor-menu-item);
var(--cursor-radio);
var(--cursor-slider-thumb);
var(--cursor-slider-thumb-active);
var(--cursor-switch);
```

## Customization

It's common to use a pointer cursor for interactive elements. Radix Themes cursors can be customized by overriding the corresponding CSS variables of the token system.

Here's an example of how you can customize the cursor tokens to set `cursor: pointer` for most interactive elements in the theme:

```css
.radix-themes {
    --cursor-button: pointer;
    --cursor-checkbox: pointer;
    --cursor-disabled: default;
    --cursor-link: pointer;
    --cursor-menu-item: pointer;
    --cursor-radio: pointer;
    --cursor-slider-thumb: grab;
    --cursor-slider-thumb-active: grabbing;
    --cursor-switch: pointer;
}
```

Make sure that your CSS is applied after the Radix Themes styles so that it takes precedence.

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/theme/cursors)
