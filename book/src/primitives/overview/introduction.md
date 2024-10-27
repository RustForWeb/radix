# Introduction

An open-source UI component library for building high-quality, accessible design systems and web apps.

Radix Primitives is a low-level UI component library with a focus on accessibility, customization and developer experience. You can use these components either as the base layer of your design system, or adopt them incrementally.

## Vision

Most of us share similar definitions for common UI patterns like accordion, checkbox, combobox, dialog, dropdown, select, slider, and tooltip. These UI patterns are [documented by WAI-ARIA](https://www.w3.org/WAI/ARIA/apg/#aria_ex). and generally understood by the community.

However, the implementations provided to us by the web platform are inadequate. They're either non-existent, lacking in functionality, or cannot be customized sufficiently.

So, developers are forced to build custom components; an incredibly difficult task. As a result, most components on the web are inaccessible, non-performant, and lacking important features.

The goal of Radix Primitives is to create an open-source component library that the community can use to build accessible design systems.

## Key Features

### Accessible

Components adhere to the [WAI-ARIA design patterns](https://www.w3.org/WAI/ARIA/apg/) where possible. It handles many of the difficult implementation details related to accessibility, including aria and role attributes, focus management, and keyboard navigation. Learn more in the [accessibility](./accessibility.md) overview.

### Unstyled

Components ship with zero styles, giving you complete control over styling. Components can be styled with any styling solution (vanilla CSS, CSS preprocessors, CSS-in-Rust libraries). Learn more in the [styling](../guides/styling.md) guide.

### Opened

Radix Primitives are designed to be customized to suit your needs. The open component architecture provides you granular access to each component part, so you can wrap them and add your own event listeners, props, or refs.

### Uncontrolled

Where applicable, components are uncontrolled by default but can also be controlled, alternatively. All of the behavior wiring is handled internally, so you can get up and running as smoothly as possible, without needing to create any local states.

### Developer Experience

One of the main goals is to provide the best possible developer experience. Radix Primitives provides a fully-typed API. All components share a similar API, creating a consistent and predictable experience. They also implement an `as_child` prop, giving users full control over the rendered element.

### Incremental Adoption

Each primitive can be installed individually so you can adopt them incrementally.

## See Also

-   [Radix documentation](https://www.radix-ui.com/primitives/docs/overview/introduction)
