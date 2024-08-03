<p align="center">
    <img src="./images/logo.svg" width="300" height="200" alt="Rust Radix Logo">
</p>

# Introduction

Rust Radix is a Rust port of [Radix](https://www.radix-ui.com/primitives).

TODO: introduction text

## Parts

Rust Radix consists of the following parts:

-   [Colors](./colors)
-   [Icons](./icons)
-   [Primitives](./primitives)

## Frameworks

Rust Radix is available for the following frameworks:

-   [Leptos](https://leptos.dev/)

The following frameworks are under consideration:

-   [Dioxus](https://dioxuslabs.com/)
-   [Yew](https://yew.rs/)

The tables below show the support for the various frameworks.

-   ✅ = Supported
-   🟦 = Early Support
-   🚧 = Work In Progress
-   ❌ = Unsupported

### Colors Support

| Name   | Framework Independent |
| ------ | --------------------- |
| Colors | ✅                    |

### Icons Support

| Name  | Dioxus | Leptos | Yew |
| ----- | ------ | ------ | --- |
| Icons | ❌     | 🚧     | 🚧  |

### Primitives Support

| Name                   | Dioxus | Leptos                                                  | Yew |
| ---------------------- | ------ | ------------------------------------------------------- | --- |
| Accessible Icon        | ❌     | 🚧 [#17](https://github.com/RustForWeb/radix/issues/17) | ❌  |
| Accordion              | ❌     | ❌ [#18](https://github.com/RustForWeb/radix/issues/18) | ❌  |
| Alert Dialog           | ❌     | ❌ [#19](https://github.com/RustForWeb/radix/issues/19) | ❌  |
| Arrow                  | ❌     | 🚧 [#20](https://github.com/RustForWeb/radix/issues/20) | ❌  |
| Aspect Ratio           | ❌     | 🟦 [#21](https://github.com/RustForWeb/radix/issues/21) | ❌  |
| Avatar                 | ❌     | 🚧 [#22](https://github.com/RustForWeb/radix/issues/22) | ❌  |
| Checkbox               | ❌     | ❌ [#23](https://github.com/RustForWeb/radix/issues/23) | ❌  |
| Collapsible            | ❌     | ❌ [#24](https://github.com/RustForWeb/radix/issues/24) | ❌  |
| Collection             | ❌     | 🟦 [#25](https://github.com/RustForWeb/radix/issues/25) | ❌  |
| Compose Refs           | ❌     | 🚧 [#26](https://github.com/RustForWeb/radix/issues/26) | ❌  |
| Context Menu           | ❌     | ❌ [#27](https://github.com/RustForWeb/radix/issues/27) | ❌  |
| Context                | ❌     | ❌ [#28](https://github.com/RustForWeb/radix/issues/28) | ❌  |
| Dialog                 | ❌     | ❌ [#29](https://github.com/RustForWeb/radix/issues/29) | ❌  |
| Direction              | ❌     | 🟦 [#30](https://github.com/RustForWeb/radix/issues/30) | ❌  |
| Dismissable Layer      | ❌     | 🚧 [#31](https://github.com/RustForWeb/radix/issues/31) | ❌  |
| Dropdown Menu          | ❌     | ❌ [#32](https://github.com/RustForWeb/radix/issues/32) | ❌  |
| Focus Guards           | ❌     | 🟦 [#33](https://github.com/RustForWeb/radix/issues/33) | ❌  |
| Focus Scope            | ❌     | 🚧 [#34](https://github.com/RustForWeb/radix/issues/34) | ❌  |
| Form                   | ❌     | ❌ [#35](https://github.com/RustForWeb/radix/issues/35) | ❌  |
| Hover Card             | ❌     | ❌ [#36](https://github.com/RustForWeb/radix/issues/36) | ❌  |
| Label                  | ❌     | 🟦 [#37](https://github.com/RustForWeb/radix/issues/37) | ❌  |
| Menu                   | ❌     | 🚧 [#38](https://github.com/RustForWeb/radix/issues/38) | ❌  |
| Menubar                | ❌     | ❌ [#39](https://github.com/RustForWeb/radix/issues/39) | ❌  |
| Navigation Menu        | ❌     | ❌ [#40](https://github.com/RustForWeb/radix/issues/40) | ❌  |
| Popover                | ❌     | ❌ [#41](https://github.com/RustForWeb/radix/issues/41) | ❌  |
| Popper                 | ❌     | 🟦 [#42](https://github.com/RustForWeb/radix/issues/42) | ❌  |
| Portal                 | ❌     | 🟦 [#43](https://github.com/RustForWeb/radix/issues/43) | ❌  |
| Presence               | ❌     | 🟦 [#44](https://github.com/RustForWeb/radix/issues/44) | ❌  |
| Primitive              | ❌     | 🟦 [#45](https://github.com/RustForWeb/radix/issues/45) | ❌  |
| Progress               | ❌     | 🟦 [#46](https://github.com/RustForWeb/radix/issues/46) | ❌  |
| Radio Group            | ❌     | ❌ [#47](https://github.com/RustForWeb/radix/issues/47) | ❌  |
| Roving Focus           | ❌     | 🚧 [#48](https://github.com/RustForWeb/radix/issues/48) | ❌  |
| Scroll Area            | ❌     | ❌ [#49](https://github.com/RustForWeb/radix/issues/49) | ❌  |
| Select                 | ❌     | ❌ [#50](https://github.com/RustForWeb/radix/issues/50) | ❌  |
| Separator              | ❌     | 🟦 [#51](https://github.com/RustForWeb/radix/issues/51) | ❌  |
| Slider                 | ❌     | ❌ [#52](https://github.com/RustForWeb/radix/issues/52) | ❌  |
| Slot                   | ❌     | 🚧 [#53](https://github.com/RustForWeb/radix/issues/53) | ❌  |
| Switch                 | ❌     | 🟦 [#54](https://github.com/RustForWeb/radix/issues/54) | ❌  |
| Tabs                   | ❌     | ❌ [#55](https://github.com/RustForWeb/radix/issues/55) | ❌  |
| Toast                  | ❌     | ❌ [#56](https://github.com/RustForWeb/radix/issues/56) | ❌  |
| Toggle Group           | ❌     | ❌ [#57](https://github.com/RustForWeb/radix/issues/57) | ❌  |
| Toggle                 | ❌     | 🚧 [#58](https://github.com/RustForWeb/radix/issues/58) | ❌  |
| Toolbar                | ❌     | ❌ [#59](https://github.com/RustForWeb/radix/issues/59) | ❌  |
| Tooltip                | ❌     | ❌ [#60](https://github.com/RustForWeb/radix/issues/60) | ❌  |
| Use Callback Ref       | ❌     | ❌                                                      | ❌  |
| Use Controllable State | ❌     | 🟦 [#61](https://github.com/RustForWeb/radix/issues/61) | ❌  |
| Use Escape Keydown     | ❌     | 🟦 [#62](https://github.com/RustForWeb/radix/issues/62) | ❌  |
| Use Layout Effect      | ❌     | ❌                                                      | ❌  |
| Use Previous           | ❌     | 🟦 [#63](https://github.com/RustForWeb/radix/issues/63) | ❌  |
| Use Rect               | ❌     | ❌ [#64](https://github.com/RustForWeb/radix/issues/64) | ❌  |
| Use Size               | ❌     | 🟦 [#65](https://github.com/RustForWeb/radix/issues/65) | ❌  |
| Visually Hidden        | ❌     | 🟦 [#66](https://github.com/RustForWeb/radix/issues/66) | ❌  |

## License

This project is available under the [MIT license](https://github.com/RustForWeb/radix/blob/main/LICENSE.md).
