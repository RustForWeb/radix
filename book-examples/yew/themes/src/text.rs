#[allow(clippy::module_inception)]
#[cfg(feature = "text")]
pub mod text;
#[cfg(feature = "text-align")]
pub mod text_align;
#[cfg(feature = "text-as")]
pub mod text_as;
#[cfg(feature = "text-color")]
pub mod text_color;
#[cfg(feature = "text-form-controls")]
pub mod text_form_controls;
#[cfg(feature = "text-formatting")]
pub mod text_formatting;
#[cfg(feature = "text-high-contrast")]
pub mod text_high_contrast;
#[cfg(feature = "text-size")]
pub mod text_size;
#[cfg(feature = "text-size-content")]
pub mod text_size_content;
#[cfg(feature = "text-size-labels")]
pub mod text_size_labels;
#[cfg(feature = "text-trim")]
pub mod text_trim;
#[cfg(feature = "text-trim-box")]
pub mod text_trim_box;
#[cfg(feature = "text-truncate")]
pub mod text_truncate;
#[cfg(feature = "text-weight")]
pub mod text_weight;
#[cfg(feature = "text-wrap")]
pub mod text_wrap;