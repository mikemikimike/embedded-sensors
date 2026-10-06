#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

pub mod humidity;
pub mod sensor;
pub mod temperature;

/// Implementation details used by exported macros.
///
/// This module is public for macro hygiene and is not a stable API.
#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use paste::paste;
}
