# Animation

Animate Radix Primitives with CSS keyframes or the Rust animation library of your choice.

Adding animation to Radix Primitives should feel similar to any other component, but there are some caveats noted here in regards to exiting animations with Rust animation libraries.

## Animating with CSS Animation

The simplest way to animate Primitives is with CSS.

You can use CSS animation to animate both mount and unmount phases. The latter is possible because the Radix Primitives will suspend unmount while your animation plays out.

```css
@keyframes fadeIn {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

@keyframes fadeOut {
    from {
        opacity: 1;
    }
    to {
        opacity: 0;
    }
}

.DialogOverlay[data-state='open'],
.DialogContent[data-state='open'] {
    animation: fadeIn 300ms ease-out;
}

.DialogOverlay[data-state='closed'],
.DialogContent[data-state='closed'] {
    animation: fadeOut 300ms ease-in;
}
```

## Delegating Unmounting for Rust Animation

TODO

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/guides/animation)
