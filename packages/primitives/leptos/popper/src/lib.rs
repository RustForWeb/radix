//! Leptos port of [Radix Popper](https://www.radix-ui.com/primitives).
//!
//! This is an internal utility, not intended for public usage.
//!
//! See [`@radix-ui/react-popper`](https://www.npmjs.com/package/@radix-ui/react-popper) for the original package.

mod popper;

pub use floating_ui_leptos::{Padding, Side};
pub use popper::*;
