#![cfg_attr(feature = "nightly", feature(doc_cfg))]

pub use windows;
pub use windows_result;

pub(crate) mod preset;

/// Process related EcoQoS toggle functions.
pub mod process;
/// Threading related EcoQoS toggle functions.
pub mod thread;

/// Find thread by it's name
#[cfg_attr(feature = "nightly", doc(cfg(feature = "find_thread")))]
#[cfg(feature = "find_thread")]
pub mod utils;
