# Styling

Radix Primitives are unstyled - and compatible with any styling solution - giving you complete control over styling.

## Styling Overview

### Functional Styles

You are in control of all aspects of styling, including functional styles. For example - by default - a [Dialog Overlay](../components/dialog.md) won't cover the entire viewport. You're responsible for adding those styles, plus any presentation styles.

### Classes

All components and their parts accept a `class` prop. This class will be passed through to the DOM element. You can use it in CSS as expected.

### Data Attributes

When components are stateful, their state will be exposed in a `data-state` attribute. For example, when an [Accordion Item](../components/accordion.md) is opened, it includes a `data-state="open"` attribute.

## Styling with CSS

### Styling a Part

You can style a component part by targeting the `class` that you provide.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use radix_leptos_accordion::*;

#[component]
fn AccordionDemo() -> impl IntoView {
    view! {
        <Accordion>
            <AccordionItem class="AccordionItem" value="item-1" />
            /* ... */
        </Accordion>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use radix_yew_accordion::*;
use yew::prelude::*;

#[function_component]
fn AccordionDemo() -> Html {
    html! {
        <Accordion>
            <AccordionItem class="AccordionItem" value="item-1" />
            /* ... */
        </Accordion>
    }
}
```

{{#endtab }}
{{#endtabs }}

### Styling a State

You can style a component state by targeting its `data-state` attribute.

```css
.AccordionItem {
    border-bottom: 1px solid gainsboro;
}

.AccordionItem[data-state='open'] {
    border-bottom-width: 2px;
}
```

## Styling with CSS-in-Rust

TODO

## Extending a Primitive

Extending a primitive is done the same way you extend any component.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
// TODO
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
// TODO
```

{{#endtab }}
{{#endtabs }}

## Summary

Radix Primitives were designed to encapsulate accessibility concerns and other complex functionalities, while ensuring you retain complete control over styling.

For convenience, stateful components include a `data-state` attribute.

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/guides/styling)
