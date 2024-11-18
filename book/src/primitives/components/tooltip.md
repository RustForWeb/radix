# Tooltip

A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book-primitives"
features = ["tooltip"]
files = ["src/tooltip.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Provider to control display delay globally.
-   Opens when the trigger is focused or hovered.
-   Closes when the trigger is activated or when pressing escape.
-   Supports custom timings.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-tooltip
```

-   [View on crates.io](https://crates.io/crates/radix-yew-tooltip)
-   [View on docs.rs](https://docs.rs/radix-yew-tooltip/latest/radix_yew_tooltip/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/tooltip)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_tooltip::*;
use yew::prelude::*;

#[component]
fn Anatomy() -> Html {
    html! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger />
                <TooltipPortal>
                    <TooltipContent>
                        <TooltipArrow />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

### Provider

Wraps your app to provide global functionality to your tooltips.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop                        | Type           | Default |
| --------------------------- | -------------- | ------- |
| `delay_duration`            | `i32`          | `700`   |
| `skip_delay_duration`       | `i32`          | `300`   |
| `disable_hoverable_content` | `Option<bool>` | -       |

{{#endtab }}
{{#endtabs }}

### Root

Contains all the parts of a tooltip.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop                        | Type             | Default |
| --------------------------- | ---------------- | ------- |
| `default_open`              | `Option<bool>`   | -       |
| `open`                      | `Option<bool>`   | -       |
| `on_open_change`            | `Callback<bool>` | -       |
| `delay_duration`            | `Option<i32>`    | -       |
| `disable_hoverable_content` | `Option<bool>`   | -       |

{{#endtab }}
{{#endtabs }}

### Trigger

The button that toggles the tooltip. By default, the `TooltipContent` will position itself against the trigger.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                                       | Default |
| ---------- | ------------------------------------------ | ------- |
| `as_child` | `Callback<TooltipTriggerChildProps, Html>` | -       |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute | Values                                         |
| -------------- | ---------------------------------------------- |
| `[data-state]` | `"closed" \| "delayed-open" \| "instant-open"` |

### Portal

When used, portals the content part into the `body`.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop            | Type                       | Default |
| --------------- | -------------------------- | ------- |
| `force_mount`   | `Option<bool>`             | -       |
| `container`     | `Option<web_sys::Element>` | -       |
| `container_ref` | `Option<NodeRef>`          | -       |

{{#endtab }}
{{#endtabs }}

### Content

The component that pops out when the tooltip is open.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop                      | Type                                               | Default             |
| ------------------------- | -------------------------------------------------- | ------------------- |
| `as_child`                | `Option<Callback<TooltipContentChildProps, Html>>` | -                   |
| `on_escape_key_down`      | `Callback<KeyboardEvent>`                          | -                   |
| `on_pointer_down_outside` | `Callback<PointerDownOutsideEvent>`                | -                   |
| `force_mount`             | `Option<bool>`                                     | -                   |
| `side`                    | `Side`                                             | `Side::Top`         |
| `side_offset`             | `f64`                                              | `0.0`               |
| `align`                   | `Align`                                            | `Align::Center`     |
| `align_offset`            | `f64`                                              | `0.0`               |
| `avoid_collisions`        | `bool`                                             | `true`              |
| `collision_boundary`      | `Vec<web_sys::Element>`                            | `vec![]`            |
| `collision_padding`       | `Padding`                                          | `Padding::All(0.0)` |
| `sticky`                  | `Sticky`                                           | `Sticky::Partial`   |
| `hide_when_detatched`     | `bool`                                             | `false`             |

{{#endtab }}
{{#endtabs }}

<div style="height: 1em;"></div>

| Data attribute | Values                                         |
| -------------- | ---------------------------------------------- |
| `[data-state]` | `"closed" \| "delayed-open" \| "instant-open"` |
| `[data-side]`  | `"left" \| "right" \| "bottom" \| "top"`       |
| `[data-align]` | `"start" \| "end" \| "center"`                 |

<div style="height: 1em;"></div>

| CSS Variable                               | Description                                                                   |
| ------------------------------------------ | ----------------------------------------------------------------------------- |
| `--radix-tooltip-content-transform-origin` | The `transform-origin` computed from the content and arrow positions/offsets. |
| `--radix-tooltip-content-available-width`  | The remaining width between the trigger and the boundary edge.                |
| `--radix-tooltip-content-available-height` | The remaining height between the trigger and the boundary edge.               |
| `--radix-tooltip-trigger-width`            | The width of the trigger.                                                     |
| `--radix-tooltip-trigger-height`           | The height of the trigger.                                                    |

### Arrow

An optional arrow element to render alongside the tooltip. This can be used to help visually link the trigger with the `TooltipContent`. Must be rendered inside `TooltipContent`.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

| Prop       | Type                                             | Default |
| ---------- | ------------------------------------------------ | ------- |
| `as_child` | `Option<Callback<TooltipArrowChildProps, Html>>` | -       |
| `width`    | `f64`                                            | `10.0`  |
| `height`   | `f64`                                            | `5.0`   |

{{#endtab }}
{{#endtabs }}

## Examples

TODO

## Accessibility

### Keyboard Interactions

| Key      | Description                                |
| -------- | ------------------------------------------ |
| `Tab`    | Opens/closes the tooltip without delay.    |
| `Space`  | If open, closes the tooltip without delay. |
| `Enter`  | If open, closes the tooltip without delay. |
| `Escape` | If open, closes the tooltip without delay. |

## Custom APIs

TODO

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/tooltip)
