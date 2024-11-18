# Dark Mode

Using appearance to manage and integrate dark mode.

## Overview

Light and dark modes are supported out of the box, allowing you to easily switch appearance without additional design or styling.

TODO: example

## Basic Usage

By default, the root [Theme](../utilities/theme.md) appearance is `Appearance::Light`. To set a different appearance pass it via the `appearance` prop. This will force the theme to use the specified setting.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_themes::{Appearance, Theme};
use yew::prelude::*;

#[function_component]
pub fn AppearanceExample() -> Html {
    html! {
        <Theme appearance={Appearance::Dark}>
            <MyApp />
        </Theme>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Inheriting System Appearance

A common requirement is to inherit the appearance setting from a user's system preferences.

This is a deceptively complex problem to solve given SSR, SSG and client side hydration considerations. To make implementation easier, we recommend integrating with a theme switching library.

### With ?

TODO: recommend a library

### With Other Libraries

Any library that supports class switching is compatible. You'll just need to ensure that the appended class names match those supported by Radix Themes:

-   `class="light"`
-   `class="light-theme"`
-   `class="dark"`
-   `class="dark-theme"`

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/theme/dark-mode)
