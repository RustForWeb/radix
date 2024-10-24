# Layout

Get the layout concerns right.

## Layout Components

Layout components are used to separate layout responsibilities from content and interactivity. This is the separation of concerns that makes your app maintainable and easy to reason about, and understanding these principles is key to building your interfaces effectively.

### Box

<!-- TODO: example -->

[Box](../layout/box.md) is the most fundamental layout component. Box is used to:

-   Provide spacing to child elements.
-   Impose sizing constraints on content.
-   Control layout behaviour within flex and grid containers.
-   Hide content based on screen size using its responsive `display` prop.

### Flex

<!-- TODO: example -->

[Flex](../layout/flex.md) component does everything that Box can do, but comes with an additional set of props to organize items along an axis. It provides convenient access to the CSS [flexbox properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/).

### Grid

<!-- TODO: example -->

[Grid](../layout/grid.md) is used to organize the content in columns and rows. Like Box and Flex, it's made to provide convenient access to the underlying CSS [grid properties](https://css-tricks.com/snippets/css/complete-guide-grid/) without any magic of its own.

### Section

<!-- TODO: example -->

[Section](../layout/section.md) provides a consistent vertical spacing between the larger parts of your page content, creating a sense of hierarchy and separation. There's just a few pre-defined sizes for different spacing levels to keep things simple and consistent.

### Container

<!-- TODO: example -->

[Container]()'s sole responsibility is to provide a consistent `max-width` to the content it wraps. Like Section, it comes just with a couple of pre-defined sizes that work well with common breakpoints and typical content widths for comfortable reading.

## Common Layout Props

Each layout component has a set of it’s own specialized props and also a shared set of common layout props. All layout props support [responsive values](../theme/breakpoints.md).

### Padding

TODO

### Width

TODO

### Height

TODO

### Positioning

TODO

### Flex Children

TODO

### Grid Children

TODO

## Margin Props

TODO

## See Also

-   [Radix documentation](https://www.radix-ui.com/themes/docs/overview/layout)
