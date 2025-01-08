
<p align="center">
    <a href="../../../../logo.svg">
        <img src="../../../../logo.svg" width="300" height="200" alt="Rust Radix Logo">
    </a>
</p>

<h1 align="center">radix-leptos-primitive</h1>

This is an internal utility, not intended for public usage.

[Rust Radix](https://github.com/RustForWeb/radix) is a Rust port of [Radix](https://www.radix-ui.com/primitives).

## Overview

```rust
use leptos::*;
use leptos_node_ref::AnyNodeRef;
use leptos_typed_fallback_show::TypedFallbackShow;

/// A generic Primitive component. Renders `element()` by default, or its
/// children directly if `as_child` is `true`. We rely on `TypedChildrenFn`
/// so that attributes can pass through at runtime—critical in Leptos v0.7
/// because `Children`-based types block such passthrough.
#[component]
#[allow(non_snake_case)]
pub fn Primitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    children: TypedChildrenFn<C>,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
) -> impl IntoView
where
    E: ElementType + 'static,
    C: IntoView + 'static,
{
    let children = StoredValue::new(children.into_inner());
    view! {
        <TypedFallbackShow
            when=move || as_child.get().unwrap_or_default()
            fallback=move || {
                element()
                    .child(children.with_value(|c| c()))
                    .add_any_attr(leptos_node_ref::any_node_ref(node_ref))
            }
        >
            {children.with_value(|c| c())
                .add_any_attr(leptos_node_ref::any_node_ref(node_ref))}
        </TypedFallbackShow>
    }
}

/// Same idea, but for elements that do not take children (e.g. `img`, `input`).
#[component]
#[allow(non_snake_case)]
pub fn VoidPrimitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    children: TypedChildrenFn<C>,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
) -> impl IntoView
where
    E: ElementType + 'static,
{
    let children = StoredValue::new(children.into_inner());
    view! {
        <TypedFallbackShow
            when=move || as_child.get().unwrap_or_default()
            fallback=move || {
                element().add_any_attr(leptos_node_ref::any_node_ref(node_ref))
            }
        >
            {children.with_value(|c| c())
                .add_any_attr(leptos_node_ref::any_node_ref(node_ref))}
        </TypedFallbackShow>
    }
}

// (Compose callbacks is an internal piece from Radix Core; omitted for brevity.)
```

## Notes

- **Why `TypedChildrenFn`?**: Leptos attribute passthrough only works if a component doesn't rely on `AnyView` or `Children`. Using typed children ensures classes, events, etc. from the parent can flow to the rendered DOM node.
- **`as_child`**: Mimics `asChild` in Radix’s React version, but we skip an explicit `<Slot>`: Leptos’s approach to typed fallback rendering covers “slot-like” logic.
- **Class Handling**: Static classes from a parent can overwrite child-defined classes. No built-in merging exists.
- **Attribute System Limitations**: Leptos limits you to 26 dynamic attributes. Past that, nest components or try a custom approach.
- **Parity with React**: In React, `...props` merges everything automatically. In Leptos, we rely on typed props/attributes and can intercept unknown ones with `AttributeInterceptor`.

## Documentation

See [the Rust Radix book](https://radix.rustforweb.org/) for documentation.

## Rust For Web

The Rust Radix project is part of the [Rust For Web](https://github.com/RustForWeb).

[Rust For Web](https://github.com/RustForWeb) creates and ports web UI libraries for Rust. All projects are free and open source.
