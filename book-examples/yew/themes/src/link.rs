#[allow(clippy::module_inception)]
#[cfg(feature = "link")]
pub mod link;
#[cfg(feature = "link-color")]
pub mod link_color;
#[cfg(feature = "link-high-contrast")]
pub mod link_high_contrast;
#[cfg(feature = "link-size")]
pub mod link_size;
#[cfg(feature = "link-truncate")]
pub mod link_truncate;
#[cfg(feature = "link-underline")]
pub mod link_underline;
#[cfg(feature = "link-weight")]
pub mod link_weight;
