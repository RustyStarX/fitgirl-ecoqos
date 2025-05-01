#![cfg_attr(feature = "nightly", feature(doc_cfg))]

pub use windows;
pub use windows_result;

pub(crate) mod preset;

/// Process related EcoQoS toggle functions.
pub mod process;
/// Threading related EcoQoS toggle functions.
pub mod thread;

/// Find thread by it's name.
///
/// For a full example to open thread after obtained Win32 ThreadID,
/// see [retrieve_thread.rs](https://github.com/mokurin000/fitgirl-ecoqos/blob/master/win32-ecoqos/examples/retrieve_thread.rs)
pub mod utils;
