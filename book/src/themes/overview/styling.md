# Styling

How to approach styling with Radix Themes.

## Introduction

Radix Themes does not come with a built-in styling system. There's no `css` or `sx` prop, and it does not use any styling libraries internally. Under the hood, it's built with vanilla CSS.

There's no overhead when it comes to picking a styling technology for your app.

## What You Get

The components in Radix Themes are relatively closed - they come with a set of styles that aren't always easily overridden. They are customizable within what's allowed by their props and the theme configuration.

However, you also get access to the same CSS variables that power the Radix Themes components. You can use these tokens to create custom components that naturally feel at home in the original theme. Changes to the token system are treated as breaking.

For more information on specific tokens, refer to the corresponding guides in the [Theme section](../theme/overview.md).

<!-- TODO: color, typography, shadow and radius examples -->

## Overriding Styles

Beyond simple style overrides, we recommend using the components as-is, or create your own versions using the same building blocks.

Most components do have `class` and `style` props, but if you find yourself needing to override a lot of styles, it's a good sign that you should either:

-   Try to achieve what you need with the existing props and theme configuration.
-   See whether you can achieve your design by tweaking the underlying token system.
-   Create your own component using lower-level building blocks, such as [Primitives](../../primitives) and [Colors](../../colors).
-   Reconsider whether Radix Themes is the right fit for your project.

### Tailwind

Tailwind is great. Yet, if you plan to use Radix Themes with Tailwind, keep in mind how its ergonomics may encourage you to create complex styles on the fly, sometimes reaching into the component internals without friction.

Tailwind is a different styling paradigm, which may not mix well with the idea of a closed component system where customization is achieved through props, tokens, and creating new components on top of a shared set of building blocks.

## Custom Components

If you need to create a custom component, use the same building blocks that Radix Themes uses:

-   [Theme](../theme/overview.md) tokens that power the components
-   [Radix Primitives](../../primitives), a library of accessible, unstyled components
-   [Radix Colors](../../colors), a color system for building beautiful websites and apps

Feel free to explore the [source code](https://github.com/RustForWeb/radix/tree/main/packages/themes) of Radix Themes to see how it is built.

## Common Issues

### Z-Index Conflicts

Out of the box, portalled Radix Themes components can be nested and stacked in any order without conflicts. For example, you can open a popover that opens a dialog, which in turn opens another popover. They all stack on top of each other in the order they were opened:

TODO: demo

When building your own components, use the following rules to avoid z-index conflicts:

-   Don't use `z-index` values other than `auto`, `0`, or `-1` in rare cases.
-   Render the elements that should stack on top of each other in portals.

Your main content and portalled content are separated by the stacking context that the styles of the root `Theme` component create. This allows you to stack portalled content on top of the main content without worrying about z-indices.

### Tailwind Base Styles

As of Tailwind v3, styles produced by the `@tailwind` directive are usually appended after any imported CSS, no matter the original import order. In particular, Tailwind's button reset style may interfere with Radix Themes buttons, rendering certain buttons without a background color.

Workarounds:

-   Don't use `@tailwind base`.
-   Set up separate CSS [layers](https://developer.mozilla.org/en-US/docs/Web/CSS/@layer) for Tailwind and Radix Themes.
-   Set up `postcss-import` and manually import Tailwind base styles via `@import tailwindcss/base` before Radix Themes styles. [Example setup](https://github.com/radix-ui/themes/issues/109#issuecomment-1747345743).

### Missing Styles in Portals

When you render a custom portal in a Radix Themes project, it will naturally appear outside of the root `Theme` component, which means it won't have access to most of the theme tokens and styles. To fix that, wrap the portal content with another `Theme`.

Components like Dialog and Popover in Radix Themes already handle this for you, so this is only necessary when creating your own portalled components.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
// Implementation example of a custom dialog using the low-level Dialog primitive
// Refer to https://radix.rustforweb.org/primitives/components/dialog.html
use leptos::*;
use radix_leptos_dialog::*;
use radix_leptos_themes::Theme;

#[component]
fn MyCustomDialog() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>Open</DialogTrigger>
            <DialogPortal>
                <Theme>
                    <DialogOverlay />
                    <DialogContent>
                        <DialogTitle />
                        <DialogDescription />
                        <DialogClose />
                    </DialogContent>
                </Theme>
            </DialogPortal>
        </Dialog>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
// Implementation example of a custom dialog using the low-level Dialog primitive
// Refer to https://radix.rustforweb.org/primitives/components/dialog.html
use radix_yew_dialog::*;
use radix_yew_themes::Theme;
use yew::prelude::*;

#[function_component]
fn MyCustomDialog() -> Html {
    html! {
        <Dialog>
            <DialogTrigger>{"Open"}</DialogTrigger>
            <DialogPortal>
                <Theme>
                    <DialogOverlay />
                    <DialogContent>
                        <DialogTitle />
                        <DialogDescription />
                        <DialogClose />
                    </DialogContent>
                </Theme>
            </DialogPortal>
        </Dialog>
    }
}
```

{{#endtab }}
{{#endtabs }}

### Complex CSS Precedence

Usually, you'd want your custom CSS to override Radix Themes styles. However, there are cases when it is natural to expect the opposite.

Consider a simple paragraph style that just resets the browser's default margin:

```css
.my-paragraph {
    margin: 0;
}
```

And the styles are imported as follows:

```css
@import '@radix-ui/themes/styles.css';
@import './my-styles.css';
```

You might apply the margin prop from a `Box` onto your custom paragraph via `as_child`:

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_dialog::*;

#[component]
fn MyApp() -> impl IntoView {
    view! {
        // TODO
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_themes::{Box, Theme};
use yew::prelude::*;

#[function_component]
fn MyApp() -> Html {
    html! {
        <Theme>
            <Box
                m=5
                as_child={Callback::from(|BoxChildProps {class, style, ..}| html! {
                    <p class={format!("{} my-paragraph", class)} style=style>{"My custom paragraph"}</p>
                })}
            />
        </Theme>
    }
}
```

{{#endtab }}
{{#endtabs }}

Yet, this won't work intuitively. The custom styles are imported after Radix Themes styles, so they will override the margin prop. As a workaround, Radix Themes provides separate `tokens.css`, `components.css`, and `utilities.css` files that the original `styles.css` is built upon:

```css
@import '@radix-ui/themes/tokens.css';
@import '@radix-ui/themes/components.css';
@import '@radix-ui/themes/utilities.css';
```

You can import `utilities.css` after your custom styles to ensure that the layout props work as expected with your custom styles.

If you use standalone layout components, split CSS files are also available for them:

```css
@import '@radix-ui/themes/layout/tokens.css';
@import '@radix-ui/themes/layout/components.css';
@import '@radix-ui/themes/layout/utilities.css';
```

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/overview/styling)
