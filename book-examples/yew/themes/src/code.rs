#[allow(clippy::module_inception)]
#[cfg(feature = "code")]
pub mod code;
#[cfg(feature = "code-color")]
pub mod code_color;
#[cfg(feature = "code-high-contrast")]
pub mod code_high_contrast;
#[cfg(feature = "code-size")]
pub mod code_size;
#[cfg(feature = "code-truncate")]
pub mod code_truncate;
#[cfg(feature = "code-variant")]
pub mod code_variant;
#[cfg(feature = "code-weight")]
pub mod code_weight;
