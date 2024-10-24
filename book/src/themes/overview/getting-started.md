# Getting Started

Install Rust Radix Themes and start building in minutes.

Rust Radix Themes is a pre-styled component library that is designed to work out of the box with minimal configuration. If you are looking for the unstyled components, go to [Rust Radix Primitives](../../primitives/README.md).

## Installation

Getting up and running is quick and easy.

### 1. Install Rust Radix Themes

Install the package from your command line.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-themes
```

-   [View on crates.io](https://crates.io/crates/radix-yew-themes)
-   [View on docs.rs](https://docs.rs/radix-yew-themes/latest/radix_yew_themes/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/themes/yew)

{{#endtab }}
{{#endtabs }}

### 2. Import the CSS file

Import the global CSS file at the root of your application.

TODO

### 3. Add the Theme component

Add `Theme` to your application, wrapping the root component inside of `body`.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_themes::Theme;
use yew::prelude::*;

#[function_component]
pub fn Root() -> Html {
    html! {
        <Theme>
            <MyApp />
        </Theme>
    }
}
```

{{#endtab }}
{{#endtabs }}

### 4. Start building

You are now ready to use Rust Radix Themes components.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_themes::{Button, Flex, FlexDirection, Text};
use yew::prelude::*;

#[function_component]
pub fn MyApp() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=2>
            <Text>{"Hello from Rust Radix Themes :)"}</Text>
            <Button>{"Let's go"}</Button>
        </Flex>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Customizing your theme

Configuration is managed and applied via the [Theme](../utilities/theme.md) component.

### Basic configuration

Pass [configuration](../utilities/theme.md) to the `Theme` to customize appearance.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_themes::{AccentColor, GrayColor, Large, Radius, Scaling, Theme};
use yew::prelude::*;

#[function_component]
pub fn Root() -> Html {
    html! {
        <Theme accent_color={AccentColor::Crimson} gray_color={GrayColor::Sand} radius={Radius::Large} scaling={Scaling::S95}>
            <MyApp />
        </Theme>
    }
}
```

{{#endtab }}
{{#endtabs }}

### Using the theme panel

TODO

### Take it further

Get the most out of Radix Themes by exploring more concepts and features.

TODO: links

-   Styling
-   Layout
-   Theme Overview
-   Color
-   Dark Mode
-   Typography

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/overview/getting-started)
