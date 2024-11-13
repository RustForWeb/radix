<p align="center">
    <img src="./images/logo.svg" width="300" height="200" alt="Rust Radix Logo">
</p>

# Introduction

Rust Radix is a Rust port of [Radix](https://www.radix-ui.com/primitives).

[Radix](https://www.radix-ui.com/) is a library of components, icons, colors, and templates for building high-quality, accessible UI.

## Parts

Rust Radix consists of the following parts:

-   [Colors](./colors)
-   [Icons](./icons)
-   [Primitives](./primitives)
-   [Themes](./themes)

## Frameworks

Rust Radix is available for the following frameworks:

-   [Leptos](https://leptos.dev/)
-   [Yew](https://yew.rs/)

The following frameworks are under consideration:

-   [Dioxus](https://dioxuslabs.com/)

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
| Icons | ❌     | 🚧     | 🟦  |

### Primitives Support

| Name                   | Dioxus                                                    | Leptos                                                  | Yew                                                       |
| ---------------------- | --------------------------------------------------------- | ------------------------------------------------------- | --------------------------------------------------------- |
| Accessible Icon        | ❌ [#120](https://github.com/RustForWeb/radix/issues/120) | 🚧 [#17](https://github.com/RustForWeb/radix/issues/17) | ❌ [#69](https://github.com/RustForWeb/radix/issues/69)   |
| Accordion              | ❌ [#121](https://github.com/RustForWeb/radix/issues/121) | ❌ [#18](https://github.com/RustForWeb/radix/issues/18) | ❌ [#70](https://github.com/RustForWeb/radix/issues/70)   |
| Alert Dialog           | ❌ [#122](https://github.com/RustForWeb/radix/issues/122) | ❌ [#19](https://github.com/RustForWeb/radix/issues/19) | ❌ [#71](https://github.com/RustForWeb/radix/issues/71)   |
| Arrow                  | ❌ [#123](https://github.com/RustForWeb/radix/issues/123) | 🚧 [#20](https://github.com/RustForWeb/radix/issues/20) | 🚧 [#72](https://github.com/RustForWeb/radix/issues/72)   |
| Aspect Ratio           | ❌ [#124](https://github.com/RustForWeb/radix/issues/124) | 🟦 [#21](https://github.com/RustForWeb/radix/issues/21) | ❌ [#73](https://github.com/RustForWeb/radix/issues/73)   |
| Avatar                 | ❌ [#125](https://github.com/RustForWeb/radix/issues/125) | 🚧 [#22](https://github.com/RustForWeb/radix/issues/22) | 🚧 [#74](https://github.com/RustForWeb/radix/issues/74)   |
| Checkbox               | ❌ [#126](https://github.com/RustForWeb/radix/issues/126) | 🚧 [#23](https://github.com/RustForWeb/radix/issues/23) | ❌ [#75](https://github.com/RustForWeb/radix/issues/75)   |
| Collapsible            | ❌ [#127](https://github.com/RustForWeb/radix/issues/127) | ❌ [#24](https://github.com/RustForWeb/radix/issues/24) | ❌ [#76](https://github.com/RustForWeb/radix/issues/76)   |
| Collection             | ❌ [#128](https://github.com/RustForWeb/radix/issues/128) | 🟦 [#25](https://github.com/RustForWeb/radix/issues/25) | 🟦 [#77](https://github.com/RustForWeb/radix/issues/77)   |
| Compose Refs           | ❌ [#129](https://github.com/RustForWeb/radix/issues/129) | 🟦 [#26](https://github.com/RustForWeb/radix/issues/26) | 🟦 [#78](https://github.com/RustForWeb/radix/issues/78)   |
| Context Menu           | ❌ [#130](https://github.com/RustForWeb/radix/issues/130) | ❌ [#27](https://github.com/RustForWeb/radix/issues/27) | ❌ [#79](https://github.com/RustForWeb/radix/issues/79)   |
| Context                | ❌ [#131](https://github.com/RustForWeb/radix/issues/131) | ❌ [#28](https://github.com/RustForWeb/radix/issues/28) | ❌ [#80](https://github.com/RustForWeb/radix/issues/80)   |
| Dialog                 | ❌ [#132](https://github.com/RustForWeb/radix/issues/132) | ❌ [#29](https://github.com/RustForWeb/radix/issues/29) | ❌ [#81](https://github.com/RustForWeb/radix/issues/81)   |
| Direction              | ❌ [#133](https://github.com/RustForWeb/radix/issues/133) | 🟦 [#30](https://github.com/RustForWeb/radix/issues/30) | 🟦 [#82](https://github.com/RustForWeb/radix/issues/82)   |
| Dismissable Layer      | ❌ [#134](https://github.com/RustForWeb/radix/issues/134) | 🚧 [#31](https://github.com/RustForWeb/radix/issues/31) | 🚧 [#83](https://github.com/RustForWeb/radix/issues/83)   |
| Dropdown Menu          | ❌ [#135](https://github.com/RustForWeb/radix/issues/135) | ❌ [#32](https://github.com/RustForWeb/radix/issues/32) | ❌ [#84](https://github.com/RustForWeb/radix/issues/84)   |
| Focus Guards           | ❌ [#136](https://github.com/RustForWeb/radix/issues/136) | 🟦 [#33](https://github.com/RustForWeb/radix/issues/33) | 🟦 [#85](https://github.com/RustForWeb/radix/issues/85)   |
| Focus Scope            | ❌ [#137](https://github.com/RustForWeb/radix/issues/137) | 🚧 [#34](https://github.com/RustForWeb/radix/issues/34) | 🚧 [#86](https://github.com/RustForWeb/radix/issues/86)   |
| Form                   | ❌ [#138](https://github.com/RustForWeb/radix/issues/138) | ❌ [#35](https://github.com/RustForWeb/radix/issues/35) | ❌ [#87](https://github.com/RustForWeb/radix/issues/87)   |
| Hover Card             | ❌ [#139](https://github.com/RustForWeb/radix/issues/139) | ❌ [#36](https://github.com/RustForWeb/radix/issues/36) | ❌ [#88](https://github.com/RustForWeb/radix/issues/88)   |
| ID                     | ❌                                                        | 🟦                                                      | 🟦                                                        |
| Label                  | ❌ [#140](https://github.com/RustForWeb/radix/issues/140) | 🟦 [#37](https://github.com/RustForWeb/radix/issues/37) | 🟦 [#89](https://github.com/RustForWeb/radix/issues/89)   |
| Menu                   | ❌ [#141](https://github.com/RustForWeb/radix/issues/141) | 🚧 [#38](https://github.com/RustForWeb/radix/issues/38) | ❌ [#90](https://github.com/RustForWeb/radix/issues/90)   |
| Menubar                | ❌ [#142](https://github.com/RustForWeb/radix/issues/142) | ❌ [#39](https://github.com/RustForWeb/radix/issues/39) | ❌ [#91](https://github.com/RustForWeb/radix/issues/91)   |
| Navigation Menu        | ❌ [#143](https://github.com/RustForWeb/radix/issues/143) | ❌ [#40](https://github.com/RustForWeb/radix/issues/40) | ❌ [#92](https://github.com/RustForWeb/radix/issues/92)   |
| Popover                | ❌ [#144](https://github.com/RustForWeb/radix/issues/144) | ❌ [#41](https://github.com/RustForWeb/radix/issues/41) | ❌ [#93](https://github.com/RustForWeb/radix/issues/93)   |
| Popper                 | ❌ [#145](https://github.com/RustForWeb/radix/issues/145) | 🟦 [#42](https://github.com/RustForWeb/radix/issues/42) | 🚧 [#94](https://github.com/RustForWeb/radix/issues/94)   |
| Portal                 | ❌ [#146](https://github.com/RustForWeb/radix/issues/146) | 🟦 [#43](https://github.com/RustForWeb/radix/issues/43) | ❌ [#95](https://github.com/RustForWeb/radix/issues/95)   |
| Presence               | ❌ [#147](https://github.com/RustForWeb/radix/issues/147) | 🟦 [#44](https://github.com/RustForWeb/radix/issues/44) | 🟦 [#96](https://github.com/RustForWeb/radix/issues/96)   |
| Primitive              | ❌ [#148](https://github.com/RustForWeb/radix/issues/148) | 🟦 [#45](https://github.com/RustForWeb/radix/issues/45) | 🟦 [#97](https://github.com/RustForWeb/radix/issues/97)   |
| Progress               | ❌ [#149](https://github.com/RustForWeb/radix/issues/150) | 🟦 [#46](https://github.com/RustForWeb/radix/issues/46) | ❌ [#98](https://github.com/RustForWeb/radix/issues/98)   |
| Radio Group            | ❌ [#150](https://github.com/RustForWeb/radix/issues/150) | ❌ [#47](https://github.com/RustForWeb/radix/issues/47) | ❌ [#99](https://github.com/RustForWeb/radix/issues/99)   |
| Roving Focus           | ❌ [#151](https://github.com/RustForWeb/radix/issues/151) | 🚧 [#48](https://github.com/RustForWeb/radix/issues/48) | ❌ [#100](https://github.com/RustForWeb/radix/issues/100) |
| Scroll Area            | ❌ [#152](https://github.com/RustForWeb/radix/issues/152) | ❌ [#49](https://github.com/RustForWeb/radix/issues/49) | ❌ [#101](https://github.com/RustForWeb/radix/issues/101) |
| Select                 | ❌ [#153](https://github.com/RustForWeb/radix/issues/153) | ❌ [#50](https://github.com/RustForWeb/radix/issues/50) | 🚧 [#102](https://github.com/RustForWeb/radix/issues/102) |
| Separator              | ❌ [#154](https://github.com/RustForWeb/radix/issues/154) | 🟦 [#51](https://github.com/RustForWeb/radix/issues/51) | 🟦 [#103](https://github.com/RustForWeb/radix/issues/103) |
| Slider                 | ❌ [#155](https://github.com/RustForWeb/radix/issues/155) | ❌ [#52](https://github.com/RustForWeb/radix/issues/52) | ❌ [#104](https://github.com/RustForWeb/radix/issues/104) |
| Slot                   | ❌ [#156](https://github.com/RustForWeb/radix/issues/156) | 🚧 [#53](https://github.com/RustForWeb/radix/issues/53) | ❌ [#105](https://github.com/RustForWeb/radix/issues/105) |
| Switch                 | ❌ [#157](https://github.com/RustForWeb/radix/issues/157) | 🟦 [#54](https://github.com/RustForWeb/radix/issues/54) | 🟦 [#106](https://github.com/RustForWeb/radix/issues/106) |
| Tabs                   | ❌ [#158](https://github.com/RustForWeb/radix/issues/158) | ❌ [#55](https://github.com/RustForWeb/radix/issues/55) | ❌ [#107](https://github.com/RustForWeb/radix/issues/107) |
| Toast                  | ❌ [#159](https://github.com/RustForWeb/radix/issues/159) | ❌ [#56](https://github.com/RustForWeb/radix/issues/56) | ❌ [#108](https://github.com/RustForWeb/radix/issues/108) |
| Toggle Group           | ❌ [#160](https://github.com/RustForWeb/radix/issues/160) | ❌ [#57](https://github.com/RustForWeb/radix/issues/57) | ❌ [#109](https://github.com/RustForWeb/radix/issues/109) |
| Toggle                 | ❌ [#161](https://github.com/RustForWeb/radix/issues/161) | 🚧 [#58](https://github.com/RustForWeb/radix/issues/58) | ❌ [#110](https://github.com/RustForWeb/radix/issues/110) |
| Toolbar                | ❌ [#162](https://github.com/RustForWeb/radix/issues/162) | ❌ [#59](https://github.com/RustForWeb/radix/issues/59) | ❌ [#111](https://github.com/RustForWeb/radix/issues/111) |
| Tooltip                | ❌ [#163](https://github.com/RustForWeb/radix/issues/163) | ❌ [#60](https://github.com/RustForWeb/radix/issues/60) | 🚧 [#112](https://github.com/RustForWeb/radix/issues/112) |
| Use Callback Ref       | ❌                                                        | ❌                                                      | ❌                                                        |
| Use Controllable State | ❌ [#164](https://github.com/RustForWeb/radix/issues/164) | 🟦 [#61](https://github.com/RustForWeb/radix/issues/61) | 🟦 [#113](https://github.com/RustForWeb/radix/issues/113) |
| Use Escape Keydown     | ❌ [#165](https://github.com/RustForWeb/radix/issues/165) | 🟦 [#62](https://github.com/RustForWeb/radix/issues/62) | ❌ [#114](https://github.com/RustForWeb/radix/issues/114) |
| Use Layout Effect      | ❌                                                        | ❌                                                      | ❌                                                        |
| Use Previous           | ❌ [#166](https://github.com/RustForWeb/radix/issues/166) | 🟦 [#63](https://github.com/RustForWeb/radix/issues/63) | 🟦 [#115](https://github.com/RustForWeb/radix/issues/115) |
| Use Rect               | ❌ [#167](https://github.com/RustForWeb/radix/issues/167) | ❌ [#64](https://github.com/RustForWeb/radix/issues/64) | ❌ [#116](https://github.com/RustForWeb/radix/issues/116) |
| Use Size               | ❌ [#168](https://github.com/RustForWeb/radix/issues/168) | 🟦 [#65](https://github.com/RustForWeb/radix/issues/65) | 🟦 [#117](https://github.com/RustForWeb/radix/issues/117) |
| Visually Hidden        | ❌ [#169](https://github.com/RustForWeb/radix/issues/169) | 🟦 [#66](https://github.com/RustForWeb/radix/issues/66) | 🟦 [#118](https://github.com/RustForWeb/radix/issues/118) |
| **Total**              | 0 / 52                                                    | 29 / 52                                                 | 21 / 52                                                   |

### Themes Support

| Name              | Dioxus | Leptos | Yew     |
| ----------------- | ------ | ------ | ------- |
| Accessible Icon   | ❌     | ❌     | ❌      |
| Alert Dialog      | ❌     | ❌     | ❌      |
| Aspect Ratio      | ❌     | ❌     | 🟦      |
| Avatar            | ❌     | ❌     | 🟦      |
| Badge             | ❌     | ❌     | ❌      |
| Base Button       | ❌     | ❌     | 🟦      |
| Blockquote        | ❌     | ❌     | 🟦      |
| Box               | ❌     | ❌     | 🟦      |
| Button            | ❌     | ❌     | 🟦      |
| Callout           | ❌     | ❌     | ❌      |
| Card              | ❌     | ❌     | ❌      |
| Checkbox Cards    | ❌     | ❌     | ❌      |
| Checkbox Group    | ❌     | ❌     | ❌      |
| Checkbox          | ❌     | ❌     | ❌      |
| Code              | ❌     | ❌     | 🟦      |
| Container         | ❌     | ❌     | 🟦      |
| Context Menu      | ❌     | ❌     | ❌      |
| Data List         | ❌     | ❌     | ❌      |
| Dialog            | ❌     | ❌     | ❌      |
| Dropdown Menu     | ❌     | ❌     | ❌      |
| Em                | ❌     | ❌     | 🟦      |
| Flex              | ❌     | ❌     | 🟦      |
| Grid              | ❌     | ❌     | 🟦      |
| Heading           | ❌     | ❌     | 🟦      |
| Hover Card        | ❌     | ❌     | ❌      |
| Icon Button       | ❌     | ❌     | 🟦      |
| Icons             | ❌     | ❌     | 🟦      |
| Inset             | ❌     | ❌     | ❌      |
| Kbd               | ❌     | ❌     | 🟦      |
| Link              | ❌     | ❌     | 🟦      |
| Popover           | ❌     | ❌     | ❌      |
| Portal            | ❌     | ❌     | ❌      |
| Progress          | ❌     | ❌     | ❌      |
| Quote             | ❌     | ❌     | 🟦      |
| Radio Cards       | ❌     | ❌     | ❌      |
| Radio Group       | ❌     | ❌     | ❌      |
| Radio             | ❌     | ❌     | ❌      |
| Reset             | ❌     | ❌     | ❌      |
| Scroll Area       | ❌     | ❌     | ❌      |
| Section           | ❌     | ❌     | 🟦      |
| Segmented Control | ❌     | ❌     | ❌      |
| Select            | ❌     | ❌     | 🚧      |
| Separator         | ❌     | ❌     | 🟦      |
| Skeleton          | ❌     | ❌     | ❌      |
| Slider            | ❌     | ❌     | ❌      |
| Slot              | ❌     | ❌     | ❌      |
| Spinner           | ❌     | ❌     | 🟦      |
| Strong            | ❌     | ❌     | 🟦      |
| Switch            | ❌     | ❌     | 🟦      |
| Table             | ❌     | ❌     | ❌      |
| Tab Nav           | ❌     | ❌     | ❌      |
| Tabs              | ❌     | ❌     | ❌      |
| Text Area         | ❌     | ❌     | 🟦      |
| Text Field        | ❌     | ❌     | 🟦      |
| Text              | ❌     | ❌     | 🟦      |
| Theme Panel       | ❌     | ❌     | ❌      |
| Theme             | ❌     | ❌     | 🟦      |
| Tooltip           | ❌     | ❌     | ❌      |
| Visually Hidden   | ❌     | ❌     | 🟦      |
| **Total**         | 0 / 59 | 0 / 59 | 28 / 59 |

## License

This project is available under the [MIT license](https://github.com/RustForWeb/radix/blob/main/LICENSE.md).

## Rust For Web

The Rust Radix project is part of the [Rust For Web](https://github.com/RustForWeb).

[Rust For Web](https://github.com/RustForWeb) creates and ports web UI libraries for Rust. All projects are free and open source.
