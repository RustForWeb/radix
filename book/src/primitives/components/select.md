# Select

<div class="warning">

This component is work in progress and not yet available for use.

</div>

Displays a list of options for the user to pick from - triggered by a button.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "radix-yew-book"
features = ["select"]
files = ["src/select.rs"]
```

{{#endtab }}
{{#endtabs }}

## Features

-   Can be controlled or uncontrolled.
-   Offers 2 positioning modes.
-   Supports items, labels, groups of items.
-   Focus is fully managed.
-   Full keyboard navigation.
-   Supports custom placeholder.
-   Typeahead support.
-   Supports Right to Left direction.

## Installation

Install the component from your command line.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```shell
cargo add radix-yew-select
```

-   [View on crates.io](https://crates.io/crates/radix-yew-select)
-   [View on docs.rs](https://docs.rs/radix-yew-select/latest/radix_yew_select/)
-   [View source](https://github.com/RustForWeb/radix/tree/main/packages/primitives/yew/select)

{{#endtab }}
{{#endtabs }}

## Anatomy

Import all parts and piece them together.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_switch::*;
use yew::prelude::*;

#[function_component]
fn Anatomy() -> Html {
    html! {
        <Select>
            <SelectTrigger>
                <SelectValue />
                <SelectIcon />
            </SelectTrigger>

            <SelectPortal>
                <SelectContent>
                    <SelectScrollUpButton />
                    <SelectViewport>
                        <SelectItem>
                            <SelectItemText />
                            <SelectItemIndicator />
                        </SelectItem>

                        <SelectGroup>
                            <SelectLabel />
                            <SelectItem>
                                <SelectItemText />
                                <SelectItemIndicator />
                            </SelectItem>
                        </SelectGroup>

                        <SelectSeparator />
                    </Select.Viewport>
                    <SelectScrollDownButton />
                    <SelectArrow />
                </SelectContent>
            </Select.Portal>
        </Select>
    }
}
```

{{#endtab }}
{{#endtabs }}

## API Reference

TODO

## Examples

TODO

## Accessibility

Adheres to the [ListBox WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/listbox/).

See the W3C [Select-Only Combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/examples/combobox-select-only/) example for more information.

### Keyboard Interactions

| Key         | Description                                                                                                                           |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `Space`     | When focus is on `SelectTrigger`, opens the select and focuses the selected item. When focus is on an item, selects the focused item. |
| `Enter`     | When focus is on `SelectTrigger`, opens the select and focuses the first item. When focus is on an item, selects the focused item.    |
| `ArrowDown` | When focus is on `SelectTrigger`, opens the select. When focus is on an item, moves focus to the next item.                           |
| `ArrowUp`   | When focus is on `SelectTrigger`, opens the select. When focus is on an item, moves focus to the previous item.                       |
| `Esc`       | Closes the select and moves focus to `Select.Trigger`.                                                                                |

### Labelling

Use the [Label](./label.md) component in order to offer a visual and accessible label for the select.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_label::*;
use radix_yew_switch::*;
use yew::prelude::*;

#[function_component]
fn Labelling() -> Html {
    html! {
        <>
            <Label>
                {"Country"}
                <Select>...</Select>
            </Label>

            // or

            <Label r#for="country">Country</Label>
            <Select>
                <SelectTrigger id="country">...</SelectTrigger>

                <SelectPortal>
                    <SelectContent>...</SelectContent>
                </Select.Portal>
            </Select>
        </>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Custom APIs

TODO

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/components/switch)
