# Breakpoints

Built-in breakpoints allow you to easily build adaptive layouts.

## Available Sizes

Each breakpoint matches a fixed screen width. Values are `min-width` based and apply when the screen width is equal or greater.

| Size                  | Description         | Width    |
| --------------------- | ------------------- | -------- |
| `Breakpoint::Initial` | Phones (portait)    | `0px`    |
| `Breakpoint::Xs`      | Phones (landscape)  | `520px`  |
| `Breakpoint::Sm`      | Tablets(portait)    | `768px`  |
| `Breakpoint::Md`      | Tablets (landscape) | `1024px` |
| `Breakpoint::Lg`      | Laptops             | `1280px` |
| `Breakpoint::Xl`      | Desktops            | `1640px` |

## Usage

Most component size and layout props will accept an additional `ResponsiveValues` struct instance for modifying the given prop across breakpoints.

Each size maps to a corresponding key, the value of each will be applied when the screen size is greater than or equal to the named breakpoint.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_themes::{Breakpoint, Heading};
use yew::prelude::*;

#[function_component]
pub fn BreakpointsExample() -> Html {
    html! {
        <Heading
            size={ResponsiveValues::from([
                (Breakpoint::Initial, 3)
                (Breakpoint::Md, 5)
                (Breakpoint::Xl, 7)
            ])}
        />
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/theme/breakpoints)
