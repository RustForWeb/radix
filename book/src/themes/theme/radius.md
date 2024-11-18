# Radius

Choosing the right radius setting in your theme.

## Theme Setting

Theme `radius` setting manages the radius factor applied to the components:

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_themes::{Radius, Theme};
use yew::prelude::*;

#[function_component]
pub fn RadiusExample() -> Html {
    html! {
        <Theme radius={Radius::Dark}>
            <MyApp />
        </Theme>
    }
}
```

{{#endtab }}
{{#endtabs }}

| Radius           | Description | Example |
| ---------------- | ----------- | ------- |
| `Radius::None`   | None        | TODO    |
| `Radius::Small`  | Small       | TODO    |
| `Radius::Medium` | Medium      | TODO    |
| `Radius::Large`  | Large       | TODO    |
| `Radius::Full`   | Full        | TODO    |

The resulting `border-radius` is contextual and differs depending on the component. For example, when set to `Radius::Full`, a [Button](../components/button.md) becomes pill-shaped, while a [Checkbox](../components/checkbox.md) will never become fully rounded to prevent any confusion between it and a [Radio](../components/radio.md).

TODO: example

## Radius Overrides

Certain components allow you to override the radius factor using their own `radius` prop.

TODO: example

Components that render panels, like [Card](../components/card.md), [Dialog](../components/dialog.md), and [Popover](../components/popover.md), among others, won't have the `radius` prop, but will inherit the radius setting from the theme. The `radius` prop is also unavailable on most text-based components.

## Radius Scale

Radius values used in the components are derived from a 6-step scale.

TODO: visual

While you can't use a specific step on a particular component directly - the `radius` prop is used to set just the radius factor - you can use the radius scale to style your own components.

Radius tokens are accessed using CSS variables. You can use these tokens to style your custom components, ensuring they are consistent with the rest of your theme.

```css
/* Radius values that automatically respond to the radius factor */
var(--radius-1);
var(--radius-2);
var(--radius-3);
var(--radius-4);
var(--radius-5);
var(--radius-6);

/* A multiplier that controls the theme radius */
var(--radius-factor);

/*
 * A variable used to calculate a fully rounded radius.
 * Usually used within a CSS `max()` function.
 */
var(--radius-full);

/*
 * A variable used to calculate radius of a thumb element.
 * Usually used within a CSS `max()` function.
 */
var(--radius-thumb);
```

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/theme/radius)
